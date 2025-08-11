use crate::application::dtos::user_dto::{
    CreateUserDto, JwtResponse, LoginRequest, LoginTelegramBotDto,
};
use crate::infrastructure::app_state::UserAppState;
use axum::{Json, extract::State};
use std::sync::Arc;

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
