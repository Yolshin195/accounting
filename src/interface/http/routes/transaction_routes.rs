use crate::infrastructure::app_state::TransactionAppState;
use crate::interface::http::handlers::transaction_handler::{create_expense_transaction, create_income_transaction, delete_transaction, find_all_transaction_by_month, find_transaction_by_id, transaction_list, update_transaction};
use axum::{Router, routing::get};
use std::sync::Arc;
use axum::routing::{delete, post, put};

pub fn get_transaction_routes(state: Arc<TransactionAppState>) -> Router {
    Router::new()
        .route("/", get(transaction_list))
        .route("/month", get(find_all_transaction_by_month))
        .route("/income", post(create_income_transaction))
        .route("/expense", post(create_expense_transaction))
        .route("/{id}", get(find_transaction_by_id))
        .route("/{id}", delete(delete_transaction))
        .route("/{id}", put(update_transaction))
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