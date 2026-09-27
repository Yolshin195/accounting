use crate::infrastructure::app_state::ReferenceAppState;
use crate::interface::http::handlers::reference_handler::*;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

pub fn reference_routes(state: Arc<ReferenceAppState>) -> Router {
    Router::new()
        .route("/", get(list_reference_descriptors))
        .route("/{kind}", get(list_references).post(create_reference))
        .route(
            "/{kind}/{code}",
            get(get_reference).put(update_reference).delete(archive_reference),
        )
        .route("/{kind}/{code}/restore", post(restore_reference))
        .with_state(state)
}
