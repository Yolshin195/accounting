use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Запрос на регистрацию
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserDto {
    pub email: String,
    #[serde(rename = "name")]
    pub username: String,
    pub password: String,
}

/// Запрос на обычный логин
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Ответ с access/refresh токенами
#[derive(Debug, Serialize, ToSchema)]
pub struct JwtResponse {
    #[serde(rename = "token")]
    pub access_token: String,
    pub refresh_token: String,
}

/// Запрос от Telegram бота
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginTelegramBotDto {
    pub client_id: String,
    pub secret: String,
    pub telegram_id: String,
    pub username: String,
}
