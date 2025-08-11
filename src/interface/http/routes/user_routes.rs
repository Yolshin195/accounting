use crate::infrastructure::app_state::UserAppState;
use crate::interface::http::handlers::user_handler;
use axum::{Router, routing::post};
use std::sync::Arc;

pub fn user_routes(state: Arc<UserAppState>) -> Router {
    Router::new()
        .route("/register", post(user_handler::register))
        .route("/login", post(user_handler::login))
        .route("/login/telegram", post(user_handler::login_telegram))
        .with_state(state)
}
