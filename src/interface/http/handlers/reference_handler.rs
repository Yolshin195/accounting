use crate::application::dtos::pagination_dto::PagedResponse;
use crate::application::dtos::reference_dto::{ErrorResponse, ReferenceItem, ReferenceListQuery};
use crate::domain::reference::{ReferenceDescriptor, ReferenceError, ReferenceRecord};
use crate::domain::user::User;
use crate::infrastructure::app_state::ReferenceAppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde_json::Value;
use std::sync::Arc;

impl IntoResponse for ReferenceError {
    fn into_response(self) -> Response {
        let (status, error, fields) = match &self {
            Self::UnknownKind(_) => (StatusCode::NOT_FOUND, "unknown_reference", Vec::new()),
            Self::NotFound { .. } => (StatusCode::NOT_FOUND, "not_found", Vec::new()),
            Self::Validation(errors) => (StatusCode::BAD_REQUEST, "validation", errors.clone()),
            Self::Conflict(_) => (StatusCode::CONFLICT, "conflict", Vec::new()),
            Self::Internal(err) => {
                log::error!("Reference request failed: {err:?}");
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

#[utoipa::path(
    get,
    path = "/references",
    tag = "References",
    responses(
        (status = 200, description = "Описание всех справочников: поля, типы, подписи. По нему клиент строит меню, таблицы и формы", body = Vec<ReferenceDescriptor>),
        (status = 401, description = "Пользователь не авторизован")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_reference_descriptors(
    State(state): State<Arc<ReferenceAppState>>,
) -> Json<Vec<ReferenceDescriptor>> {
    Json(state.reference_service.descriptors())
}

#[utoipa::path(
    get,
    path = "/references/{kind}",
    tag = "References",
    params(
        ("kind" = String, Path, description = "Справочник, например `currencies` или `accounts`"),
        ReferenceListQuery
    ),
    responses(
        (status = 200, description = "Список записей справочника", body = PagedResponse<ReferenceItem>),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Справочник не найден", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_references(
    State(state): State<Arc<ReferenceAppState>>,
    Extension(user): Extension<User>,
    Path(kind): Path<String>,
    Query(query): Query<ReferenceListQuery>,
) -> Result<Json<PagedResponse<ReferenceRecord>>, ReferenceError> {
    Ok(Json(state.reference_service.list(&kind, user.id, query).await?))
}

#[utoipa::path(
    get,
    path = "/references/{kind}/{code}",
    tag = "References",
    params(
        ("kind" = String, Path, description = "Справочник"),
        ("code" = String, Path, description = "Код записи")
    ),
    responses(
        (status = 200, description = "Запись справочника", body = ReferenceItem),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Справочник или запись не найдены", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_reference(
    State(state): State<Arc<ReferenceAppState>>,
    Extension(user): Extension<User>,
    Path((kind, code)): Path<(String, String)>,
) -> Result<Json<ReferenceRecord>, ReferenceError> {
    Ok(Json(state.reference_service.get(&kind, user.id, &code).await?))
}

#[utoipa::path(
    post,
    path = "/references/{kind}",
    tag = "References",
    params(("kind" = String, Path, description = "Справочник")),
    request_body(content = ReferenceItem, description = "Базовые поля + поля справочника; `id` и `isArchived` игнорируются"),
    responses(
        (status = 201, description = "Запись создана", body = ReferenceItem),
        (status = 400, description = "Ошибка валидации", body = ErrorResponse),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Справочник не найден", body = ErrorResponse),
        (status = 409, description = "Запись с таким кодом уже существует", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_reference(
    State(state): State<Arc<ReferenceAppState>>,
    Extension(user): Extension<User>,
    Path(kind): Path<String>,
    Json(payload): Json<Value>,
) -> Result<(StatusCode, Json<ReferenceRecord>), ReferenceError> {
    let created = state.reference_service.create(&kind, user.id, payload).await?;
    Ok((StatusCode::CREATED, Json(created)))
}

#[utoipa::path(
    put,
    path = "/references/{kind}/{code}",
    tag = "References",
    params(
        ("kind" = String, Path, description = "Справочник"),
        ("code" = String, Path, description = "Код записи")
    ),
    request_body(content = ReferenceItem, description = "Полное обновление; `code` изменить нельзя"),
    responses(
        (status = 200, description = "Запись обновлена", body = ReferenceItem),
        (status = 400, description = "Ошибка валидации", body = ErrorResponse),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Справочник или запись не найдены", body = ErrorResponse),
        (status = 409, description = "Конфликт уникальности", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_reference(
    State(state): State<Arc<ReferenceAppState>>,
    Extension(user): Extension<User>,
    Path((kind, code)): Path<(String, String)>,
    Json(payload): Json<Value>,
) -> Result<Json<ReferenceRecord>, ReferenceError> {
    Ok(Json(state.reference_service.update(&kind, user.id, &code, payload).await?))
}

#[utoipa::path(
    delete,
    path = "/references/{kind}/{code}",
    tag = "References",
    params(
        ("kind" = String, Path, description = "Справочник"),
        ("code" = String, Path, description = "Код записи")
    ),
    responses(
        (status = 204, description = "Запись перенесена в архив"),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Справочник или запись не найдены", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn archive_reference(
    State(state): State<Arc<ReferenceAppState>>,
    Extension(user): Extension<User>,
    Path((kind, code)): Path<(String, String)>,
) -> Result<StatusCode, ReferenceError> {
    state.reference_service.archive(&kind, user.id, &code).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/references/{kind}/{code}/restore",
    tag = "References",
    params(
        ("kind" = String, Path, description = "Справочник"),
        ("code" = String, Path, description = "Код записи")
    ),
    responses(
        (status = 200, description = "Запись восстановлена из архива", body = ReferenceItem),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Справочник или запись не найдены", body = ErrorResponse),
        (status = 409, description = "Конфликт уникальности", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn restore_reference(
    State(state): State<Arc<ReferenceAppState>>,
    Extension(user): Extension<User>,
    Path((kind, code)): Path<(String, String)>,
) -> Result<Json<ReferenceRecord>, ReferenceError> {
    Ok(Json(state.reference_service.restore(&kind, user.id, &code).await?))
}
