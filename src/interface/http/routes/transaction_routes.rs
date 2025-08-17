use crate::infrastructure::app_state::TransactionAppState;
use crate::interface::http::handlers::transaction_handler::{create_expense_transaction, create_income_transaction, delete_transaction, transaction_list};
use axum::{Router, routing::get};
use std::sync::Arc;
use axum::routing::{delete, post};

pub fn get_transaction_routes(state: Arc<TransactionAppState>) -> Router {
    Router::new()
        .route("/", get(transaction_list))
        .route("/income", post(create_income_transaction))
        .route("/expense", post(create_expense_transaction))
        .route("/{id}", delete(delete_transaction))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http;
    use tower::ServiceExt;
    use super::*;
    use crate::application::services::transaction_service::TransactionService;
    use crate::infrastructure::db::db::init_pg_pool;
    use crate::infrastructure::db::postgres_category_repository::PostgresCategoryRepo;
    use crate::infrastructure::db::postgres_transaction_repository::PostgresTransactionRepo;

    #[tokio::test]
    async fn test_get_transaction_list() {
        // Given
        let db_pool = init_pg_pool().await.unwrap();
        let app_state = TransactionAppState {
            transaction_service: TransactionService::new(
                PostgresTransactionRepo {
                    pool: db_pool.clone()
                },
                PostgresCategoryRepo {
                    pool: db_pool.clone()
                }
            ),
        };
        let app = get_transaction_routes(Arc::new(app_state));

        // When
        let response = app.oneshot(
            http::Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap()
        ).await.unwrap();

        // Then
        assert_eq!(response.status(), http::StatusCode::OK);
    }
}