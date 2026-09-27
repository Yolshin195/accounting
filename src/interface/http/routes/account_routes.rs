use crate::infrastructure::app_state::AccountAppState;
use crate::interface::http::handlers::account_handler::*;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

pub fn account_routes(state: Arc<AccountAppState>) -> Router {
    Router::new()
        .route("/", get(list_accounts).post(create_account))
        .route("/{code}", get(get_account).put(update_account).delete(archive_account))
        .route("/{code}/restore", post(restore_account))
        .with_state(state)
}
