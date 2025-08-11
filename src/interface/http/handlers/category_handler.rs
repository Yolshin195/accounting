use crate::application::dtos::category_dto::{CategoryDto, CreateCategoryDto};
use crate::domain::user::User;
use crate::infrastructure::app_state::CategoryAppState;
use axum::{Extension, Json, extract::State};
use std::sync::Arc;

pub async fn create_category(
    State(state): State<Arc<CategoryAppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateCategoryDto>,
) -> Json<CategoryDto> {
    println!("{:?}", user);
    let created = state
        .category_service
        .create(payload, user.id)
        .await
        .unwrap(); // добавь обработку ошибок
    Json(created)
}

pub async fn list_categories(
    State(state): State<Arc<CategoryAppState>>,
    Extension(user): Extension<User>,
) -> Json<Vec<CategoryDto>> {
    println!("{:?}", user);
    let list = state.category_service.get_all(user.id).await.unwrap();
    Json(list)
}
