use crate::application::dtos::category_dto::{CategoryDto, CreateCategoryDto};
use crate::domain::user::User;
use crate::infrastructure::app_state::CategoryAppState;
use axum::{Extension, Json, extract::State, extract::Query};
use std::sync::Arc;
use axum::extract::Path;
use axum::http::StatusCode;
use crate::application::dtos::pagination_dto::{PagedResponse, Pagination};

#[utoipa::path(
    post,
    path = "/categories",
    tag = "Categories",
    request_body = CreateCategoryDto,
    responses(
        (status = 200, description = "Категория успешно создана", body = CategoryDto),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
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


#[utoipa::path(
    get,
    path = "/categories",
    tag = "Categories",
    params(
        Pagination
    ),
    responses(
        (status = 200, description = "Получен список категорий", body = PagedResponse<CategoryDto>),
        (status = 401, description = "Пользователь не авторизован")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_categories(
    State(state): State<Arc<CategoryAppState>>,
    Query(pagination): Query<Pagination>,
    Extension(user): Extension<User>,
) -> Json<PagedResponse<CategoryDto>> {
    let page = state.category_service.get_all(user.id, pagination).await.unwrap();
    Json(page)
}


#[utoipa::path(
    delete,
    path = "/categories/{code}",
    tag = "Categories",
    params(
        ("code" = String, Path, description = "Уникальный код категории для удаления")
    ),
    responses(
        (status = 204, description = "Категория успешно удалена"),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Категория не найдена"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
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
