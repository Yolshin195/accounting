use axum::{Router, routing::{get, post}};
use crate::interface::http::handlers::category_handler::*;
use std::sync::Arc;
use crate::infrastructure::app_state::CategoryAppState;

pub fn category_routes(state: Arc<CategoryAppState>) -> Router {
    Router::new()
        .route("/", post(create_category))
        .route("/", get(list_categories))
        .with_state(state)
}
