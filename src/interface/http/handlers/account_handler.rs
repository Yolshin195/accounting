use crate::application::dtos::account_dto::{AccountDto, CreateAccountDto, UpdateAccountDto};
use crate::application::dtos::pagination_dto::PagedResponse;
use crate::application::dtos::reference_dto::{ErrorResponse, ReferenceListQuery};
use crate::domain::reference::ReferenceError;
use crate::domain::user::User;
use crate::infrastructure::app_state::AccountAppState;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/accounts",
    tag = "References",
    params(ReferenceListQuery),
    responses(
        (status = 200, description = "Список счетов", body = PagedResponse<AccountDto>),
        (status = 401, description = "Пользователь не авторизован")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_accounts(
    State(state): State<Arc<AccountAppState>>,
    Extension(user): Extension<User>,
    Query(query): Query<ReferenceListQuery>,
) -> Result<Json<PagedResponse<AccountDto>>, ReferenceError> {
    Ok(Json(state.account_service.list(user.id, query).await?))
}

#[utoipa::path(
    get,
    path = "/accounts/{code}",
    tag = "References",
    params(("code" = String, Path, description = "Код счёта")),
    responses(
        (status = 200, description = "Счёт", body = AccountDto),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Счёт не найден", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_account(
    State(state): State<Arc<AccountAppState>>,
    Extension(user): Extension<User>,
    Path(code): Path<String>,
) -> Result<Json<AccountDto>, ReferenceError> {
    Ok(Json(state.account_service.get(user.id, &code).await?))
}

#[utoipa::path(
    post,
    path = "/accounts",
    tag = "References",
    request_body = CreateAccountDto,
    responses(
        (status = 201, description = "Счёт создан", body = AccountDto),
        (status = 400, description = "Ошибка валидации", body = ErrorResponse),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 409, description = "Счёт с таким кодом уже существует", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_account(
    State(state): State<Arc<AccountAppState>>,
    Extension(user): Extension<User>,
    payload: Result<Json<CreateAccountDto>, JsonRejection>,
) -> Result<(StatusCode, Json<AccountDto>), ReferenceError> {
    let Json(dto) = payload?;
    Ok((StatusCode::CREATED, Json(state.account_service.create(user.id, dto).await?)))
}

#[utoipa::path(
    put,
    path = "/accounts/{code}",
    tag = "References",
    params(("code" = String, Path, description = "Код счёта")),
    request_body = UpdateAccountDto,
    responses(
        (status = 200, description = "Счёт обновлён", body = AccountDto),
        (status = 400, description = "Ошибка валидации", body = ErrorResponse),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Счёт не найден", body = ErrorResponse),
        (status = 409, description = "Конфликт уникальности", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_account(
    State(state): State<Arc<AccountAppState>>,
    Extension(user): Extension<User>,
    Path(code): Path<String>,
    payload: Result<Json<UpdateAccountDto>, JsonRejection>,
) -> Result<Json<AccountDto>, ReferenceError> {
    let Json(dto) = payload?;
    Ok(Json(state.account_service.update(user.id, &code, dto).await?))
}

#[utoipa::path(
    delete,
    path = "/accounts/{code}",
    tag = "References",
    params(("code" = String, Path, description = "Код счёта")),
    responses(
        (status = 204, description = "Счёт перенесён в архив"),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Счёт не найден", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn archive_account(
    State(state): State<Arc<AccountAppState>>,
    Extension(user): Extension<User>,
    Path(code): Path<String>,
) -> Result<StatusCode, ReferenceError> {
    state.account_service.archive(user.id, &code).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/accounts/{code}/restore",
    tag = "References",
    params(("code" = String, Path, description = "Код счёта")),
    responses(
        (status = 200, description = "Счёт восстановлен из архива", body = AccountDto),
        (status = 401, description = "Пользователь не авторизован"),
        (status = 404, description = "Счёт не найден", body = ErrorResponse),
        (status = 409, description = "Конфликт уникальности", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn restore_account(
    State(state): State<Arc<AccountAppState>>,
    Extension(user): Extension<User>,
    Path(code): Path<String>,
) -> Result<Json<AccountDto>, ReferenceError> {
    Ok(Json(state.account_service.restore(user.id, &code).await?))
}
