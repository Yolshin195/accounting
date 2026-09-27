use crate::application::dtos::reference_dto::ErrorResponse;
use crate::domain::reference::ReferenceError;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

impl IntoResponse for ReferenceError {
    fn into_response(self) -> Response {
        let (status, error, fields) = match &self {
            Self::NotFound(_) => (StatusCode::NOT_FOUND, "not_found", Vec::new()),
            Self::Validation(errors) => (StatusCode::BAD_REQUEST, "validation", errors.clone()),
            Self::Conflict(_) => (StatusCode::CONFLICT, "conflict", Vec::new()),
            Self::Internal(err) => {
                log::error!("Request failed: {err:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal", Vec::new())
            }
        };
        let message = match &self {
            Self::Internal(_) => "Internal server error".to_string(),
            _ => self.to_string(),
        };
        (status, Json(ErrorResponse { error, message, fields })).into_response()
    }
}

/// Некорректный JSON (не тот тип, неизвестное значение enum) — тоже ошибка валидации,
/// чтобы клиент всегда получал `ErrorResponse`. В хендлере:
/// `payload: Result<Json<Dto>, JsonRejection>` и `let Json(dto) = payload?;`
impl From<JsonRejection> for ReferenceError {
    fn from(rejection: JsonRejection) -> Self {
        ReferenceError::invalid("body", rejection.body_text())
    }
}
