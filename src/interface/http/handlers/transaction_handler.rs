use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::{Path, Query, State};
use axum::http::{StatusCode};
use uuid::Uuid;
use crate::application::dtos::pagination_dto::{PagedResponse, Pagination};
use crate::application::dtos::transaction_dto::{CategoryExpenseSummaryDto, CreateTransactionDto, MonthlyTransactionQuery, TransactionDto, UpdateTransactionDto};
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

pub async fn find_all_transaction_by_month(
    State(state): State<Arc<TransactionAppState>>,
    Query(monthly_transaction_query): Query<MonthlyTransactionQuery>,
    Extension(user): Extension<User>,
) -> Json<PagedResponse<TransactionDto>> {
    let page = state.transaction_service.find_all_by_month(
        user.id,
        monthly_transaction_query.year,
        monthly_transaction_query.month,
        monthly_transaction_query.pagination
    ).await.unwrap();
    Json(page)
}

pub async fn update_transaction(
    State(state): State<Arc<TransactionAppState>>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(transaction): Json<UpdateTransactionDto>,
) -> Result<Json<TransactionDto>, StatusCode>{
    match state.transaction_service.update(user.id, id, transaction).await {
        Ok(transaction_dto) => Ok(Json(transaction_dto)),
        Err(err) => {
            if err.to_string().contains("not found") {
                Err(StatusCode::NOT_FOUND) // 404 - не найдено
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR) // 500 - другая ошибка
            }
        },
    }
}

pub async fn find_transaction_by_id(
    State(state): State<Arc<TransactionAppState>>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Result<Json<TransactionDto>, StatusCode> {
    match state.transaction_service.find_by_user_id_and_id(user.id, id).await {
        Ok(transaction_dto) => Ok(Json(transaction_dto)),
        Err(err) => {
            if err.to_string().contains("not found") {
                Err(StatusCode::NOT_FOUND) // 404 - не найдено
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR) // 500 - другая ошибка
            }
        },
    }
}

pub async fn sum_today_expenses_grouped_by_category(
    State(state): State<Arc<TransactionAppState>>,
    Extension(user): Extension<User>,
) -> Json<Vec<CategoryExpenseSummaryDto>> {
    let rows = state.transaction_service.sum_today_expenses_grouped_by_category(user.id).await.unwrap();
    Json(rows)
}
