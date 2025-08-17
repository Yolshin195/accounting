use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::{Path, Query, State};
use axum::http::{StatusCode};
use uuid::Uuid;
use crate::application::dtos::pagination_dto::{PagedResponse, Pagination};
use crate::application::dtos::transaction_dto::{CreateTransactionDto, TransactionDto};
use crate::domain::user::User;
use crate::infrastructure::app_state::{TransactionAppState};

pub async fn transaction_list(
    State(state): State<Arc<TransactionAppState>>,
    Query(pagination): Query<Pagination>,
    Extension(user): Extension<User>,
) -> Json<PagedResponse<TransactionDto>> {
    let page = state.transaction_service.get_all(user.id, pagination).await.unwrap();
    Json(page)
}

pub async fn create_income_transaction(
    State(state): State<Arc<TransactionAppState>>,
    Extension(user): Extension<User>,
    Json(transaction): Json<CreateTransactionDto>,
) -> Json<TransactionDto> {
    let transaction = state.transaction_service.create_income(user.id, transaction).await.unwrap();
    Json(transaction)
}

pub async fn create_expense_transaction(
    State(state): State<Arc<TransactionAppState>>,
    Extension(user): Extension<User>,
    Json(transaction): Json<CreateTransactionDto>,
) -> Json<TransactionDto> {
    let transaction = state.transaction_service.create_expense(user.id, transaction).await.unwrap();
    Json(transaction)
}

pub async fn delete_transaction(
    State(state): State<Arc<TransactionAppState>>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    match state.transaction_service.delete(user.id, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            if err.to_string().contains("not found") {
                Err(StatusCode::NOT_FOUND) // 404 - не найдено
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR) // 500 - другая ошибка
            }
        },
    }
}
