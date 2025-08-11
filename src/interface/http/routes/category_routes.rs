use crate::infrastructure::app_state::CategoryAppState;
use crate::interface::http::handlers::category_handler::*;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn category_routes(state: Arc<CategoryAppState>) -> Router {
    Router::new()
        .route("/", post(create_category))
        .route("/", get(list_categories))
        .with_state(state)
}
