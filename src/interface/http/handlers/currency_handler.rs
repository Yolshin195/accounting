use crate::application::dtos::currency_dto::{CreateCurrencyDto, CurrencyDto, UpdateCurrencyDto};
use crate::application::dtos::pagination_dto::PagedResponse;
use crate::application::dtos::reference_dto::{ErrorResponse, ReferenceListQuery};
use crate::domain::reference::ReferenceError;
use crate::domain::user::User;
use crate::infrastructure::app_state::CurrencyAppState;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/currencies",
    tag = "References",
    params(ReferenceListQuery),
    responses(
        (status = 200, description = "Список валют", body = PagedResponse<CurrencyDto>),
        (status = 401, description = "Пользователь не авторизован")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_currencies(
    State(state): State<Arc<CurrencyAppState>>,
    Extension(user): Extension<User>,
    Query(query): Query<ReferenceListQuery>,
) -> Result<Json<PagedResponse<CurrencyDto>>, ReferenceError> {
    Ok(Json(state.currency_service.list(user.id, query).await?))
}

#[utoipa::path(
    get,
    path = "/currencies/{code}",
    tag = "References",
    params(("code" = String, Path, description = "Код валюты")),
    responses(
        (status = 200, description = "Валюта", body = CurrencyDto),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Валюта не найдена", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_currency(
    State(state): State<Arc<CurrencyAppState>>,
    Extension(user): Extension<User>,
    Path(code): Path<String>,
) -> Result<Json<CurrencyDto>, ReferenceError> {
    Ok(Json(state.currency_service.get(user.id, &code).await?))
}

#[utoipa::path(
    post,
    path = "/currencies",
    tag = "References",
    request_body = CreateCurrencyDto,
    responses(
        (status = 201, description = "Валюта создана", body = CurrencyDto),
        (status = 400, description = "Ошибка валидации", body = ErrorResponse),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 409, description = "Код занят или уже есть основная валюта", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_currency(
    State(state): State<Arc<CurrencyAppState>>,
    Extension(user): Extension<User>,
    payload: Result<Json<CreateCurrencyDto>, JsonRejection>,
) -> Result<(StatusCode, Json<CurrencyDto>), ReferenceError> {
    let Json(dto) = payload?;
    Ok((StatusCode::CREATED, Json(state.currency_service.create(user.id, dto).await?)))
}

#[utoipa::path(
    put,
    path = "/currencies/{code}",
    tag = "References",
    params(("code" = String, Path, description = "Код валюты")),
    request_body = UpdateCurrencyDto,
    responses(
        (status = 200, description = "Валюта обновлена", body = CurrencyDto),
        (status = 400, description = "Ошибка валидации", body = ErrorResponse),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Валюта не найдена", body = ErrorResponse),
        (status = 409, description = "Уже есть основная валюта", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_currency(
    State(state): State<Arc<CurrencyAppState>>,
    Extension(user): Extension<User>,
    Path(code): Path<String>,
    payload: Result<Json<UpdateCurrencyDto>, JsonRejection>,
) -> Result<Json<CurrencyDto>, ReferenceError> {
    let Json(dto) = payload?;
    Ok(Json(state.currency_service.update(user.id, &code, dto).await?))
}

#[utoipa::path(
    delete,
    path = "/currencies/{code}",
    tag = "References",
    params(("code" = String, Path, description = "Код валюты")),
    responses(
        (status = 204, description = "Валюта перенесена в архив"),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Валюта не найдена", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn archive_currency(
    State(state): State<Arc<CurrencyAppState>>,
    Extension(user): Extension<User>,
    Path(code): Path<String>,
) -> Result<StatusCode, ReferenceError> {
    state.currency_service.archive(user.id, &code).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/currencies/{code}/restore",
    tag = "References",
    params(("code" = String, Path, description = "Код валюты")),
    responses(
        (status = 200, description = "Валюта восстановлена из архива", body = CurrencyDto),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Валюта не найдена", body = ErrorResponse),
        (status = 409, description = "Уже есть основная валюта", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn restore_currency(
    State(state): State<Arc<CurrencyAppState>>,
    Extension(user): Extension<User>,
    Path(code): Path<String>,
) -> Result<Json<CurrencyDto>, ReferenceError> {
    Ok(Json(state.currency_service.restore(user.id, &code).await?))
}
