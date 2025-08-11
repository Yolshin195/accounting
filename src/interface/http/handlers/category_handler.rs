use axum::{extract::State, Extension, Json};
use crate::application::dtos::category_dto::{CreateCategoryDto, CategoryDto};
use std::sync::Arc;
use crate::domain::user::User;
use crate::infrastructure::app_state::CategoryAppState;

pub async fn create_category(
    State(state): State<Arc<CategoryAppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateCategoryDto>,
) -> Json<CategoryDto> {
    println!("{:?}", user);
    let created = state.category_service.create(payload, user.id).await.unwrap(); // добавь обработку ошибок
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
