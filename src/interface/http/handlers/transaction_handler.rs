use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::{Query, State};
use crate::application::dtos::pagination_dto::{PagedResponse, Pagination};
use crate::application::dtos::transaction_dto::TransactionDto;
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
