use crate::application::dtos::category_dto::{CategoryDto, CreateCategoryDto};
use crate::domain::user::User;
use crate::infrastructure::app_state::CategoryAppState;
use axum::{Extension, Json, extract::State, extract::Query};
use std::sync::Arc;
use crate::application::dtos::pagination_dto::{PagedResponse, Pagination};

pub async fn create_category(
    State(state): State<Arc<CategoryAppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateCategoryDto>,
) -> Json<CategoryDto> {
    let created = state
        .category_service
        .create(payload, user.id)
        .await
        .unwrap(); // добавь обработку ошибок
    Json(created)
}

pub async fn list_categories(
    State(state): State<Arc<CategoryAppState>>,
    Query(pagination): Query<Pagination>,
    Extension(user): Extension<User>,
) -> Json<PagedResponse<CategoryDto>> {
    let page = state.category_service.get_all(user.id, pagination).await.unwrap();
    Json(page)
}
