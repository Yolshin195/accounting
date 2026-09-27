use crate::infrastructure::app_state::CurrencyAppState;
use crate::interface::http::handlers::currency_handler::*;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

pub fn currency_routes(state: Arc<CurrencyAppState>) -> Router {
    Router::new()
        .route("/", get(list_currencies).post(create_currency))
        .route("/{code}", get(get_currency).put(update_currency).delete(archive_currency))
        .route("/{code}/restore", post(restore_currency))
        .with_state(state)
}
