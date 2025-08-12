use crate::application::dtos::category_dto::{CategoryDto, CreateCategoryDto};
use crate::domain::user::User;
use crate::infrastructure::app_state::CategoryAppState;
use axum::{Extension, Json, extract::State, extract::Query};
use std::sync::Arc;
use axum::extract::Path;
use axum::http::StatusCode;
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

pub async fn delete_category_by_code(
    State(state): State<Arc<CategoryAppState>>,
    Extension(user): Extension<User>,
    Path(code): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match state
        .category_service
        .delete_by_code(user.id, code)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            // Проверяем, содержит ли сообщение об ошибке "not found"
            if err.to_string().contains("not found") {
                Err(StatusCode::NOT_FOUND) // 404 - не найдено
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR) // 500 - другая ошибка
            }
        }
    }
}
