use crate::application::dtos::user_dto::{
    CreateUserDto, JwtResponse, LoginRequest, LoginTelegramBotDto,
};
use crate::infrastructure::app_state::UserAppState;
use axum::{Json, extract::State};
use std::sync::Arc;


#[utoipa::path(
    post,
    path = "/users/register",
    tag = "Users",
    request_body = CreateUserDto,
    responses(
        (status = 200, description = "Пользователь успешно зарегистрирован", body = JwtResponse),
        (status = 500, description = "Внутренняя ошибка сервера")
    )
)]
pub async fn register(
    State(state): State<Arc<UserAppState>>,
    Json(payload): Json<CreateUserDto>,
) -> Result<Json<JwtResponse>, axum::http::StatusCode> {
    println!("Received registration request: {:?}", payload);
    let tokens = state
        .auth_service
        .register(&payload.username, &payload.password)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(tokens))
}


#[utoipa::path(
    post,
    path = "/users/login",
    tag = "Users",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Успешная аутентификация", body = JwtResponse),
        (status = 401, description = "Неверный логин или пароль")
    )
)]
pub async fn login(
    State(state): State<Arc<UserAppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<JwtResponse>, axum::http::StatusCode> {
    let tokens = state
        .auth_service
        .login(&payload.username, &payload.password)
        .await
        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    Ok(Json(tokens))
}


#[utoipa::path(
    post,
    path = "/users/login/telegram",
    tag = "Users",
    request_body = LoginTelegramBotDto,
    responses(
        (status = 200, description = "Успешная аутентификация через Telegram", body = JwtResponse),
        (status = 401, description = "Ошибка авторизации Telegram")
    )
)]
pub async fn login_telegram(
    State(state): State<Arc<UserAppState>>,
    Json(payload): Json<LoginTelegramBotDto>,
) -> Result<Json<JwtResponse>, axum::http::StatusCode> {
    let tokens = state
        .auth_service
        .login_telegram(&payload)
        .await
        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    Ok(Json(tokens))
}
