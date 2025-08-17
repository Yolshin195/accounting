mod application;
mod domain;
mod infrastructure;
mod interface;

use crate::application::services::auth_service::AuthService;
use crate::application::services::category_service::CategoryService;
use crate::infrastructure::app_state::{CategoryAppState, TransactionAppState};
use crate::infrastructure::app_state::UserAppState;
use crate::infrastructure::auth::jwt::JwtService;
use crate::infrastructure::db::db::init_pg_pool;
use crate::infrastructure::db::postgres_category_repository::PostgresCategoryRepo;
use crate::infrastructure::db::postgres_user_repository::PostgresUserRepository;
use crate::interface::http::middleware::auth_middleware::{JwtMiddlewareState, jwt_middleware};
use crate::interface::http::routes::category_routes::category_routes;
use crate::interface::http::routes::user_routes::user_routes;
use axum::http::Method;
use axum::{Router, http, middleware};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use crate::application::services::transaction_service::TransactionService;
use crate::infrastructure::db::postgres_transaction_repository::PostgresTransactionRepo;
use crate::interface::http::routes::transaction_routes::get_transaction_routes;

fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
        ])
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Инициализация логгера - добавьте эту строку
    env_logger::init();

    let db_pool = init_pg_pool().await?;
    let user_repo = PostgresUserRepository::new(db_pool.clone());
    let jwt_service = JwtService::new(env::var("JWT_SECRET")?.to_string(), 120, 7);
    let auth_service = AuthService::new(user_repo.clone(), jwt_service.clone());
    let user_app_state = UserAppState {
        auth_service: auth_service.clone(),
    };

    let category_repo = PostgresCategoryRepo {
        pool: db_pool.clone(),
    };
    let category_service = CategoryService::new(category_repo.clone());
    let category_app_state = CategoryAppState {
        category_service: category_service.clone(),
    };
    
    let transaction_repo = PostgresTransactionRepo { pool: db_pool.clone() };
    let transaction_service = TransactionService::new(
        transaction_repo.clone(), category_repo.clone()
    );
    let transaction_app_state = TransactionAppState {
        transaction_service
    };
    
    let jwt_middleware_state = Arc::new(JwtMiddlewareState {
        jwt_service: jwt_service.clone(),
        user_repo: Arc::new(user_repo.clone()),
    });

    let private_router = Router::new()
        .nest("/categories", category_routes(Arc::new(category_app_state)))
        .nest("/transactions", get_transaction_routes(Arc::new(transaction_app_state)))
        .layer(middleware::from_fn_with_state(
            jwt_middleware_state,
            jwt_middleware,
        ));

    let public_router = Router::new().nest("/users", user_routes(Arc::new(user_app_state)));

    let app = Router::new()
        .merge(private_router)
        .merge(public_router)
        .layer(create_cors_layer())
        .layer(TraceLayer::new_for_http());

    println!("Server starting on http://0.0.0.0:8888");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
