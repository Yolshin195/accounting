use crate::domain::user::User;
use crate::infrastructure::auth::password;

use crate::application::dtos::user_dto::{JwtResponse, LoginTelegramBotDto};
use crate::application::traits::user_repo::UserRepository;
use crate::infrastructure::auth::jwt::JwtService;
use anyhow::{Result, bail};
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthService<R: UserRepository> {
    repo: R,
    jwt: JwtService,
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(repo: R, jwt: JwtService) -> Self {
        Self { repo, jwt }
    }

    pub async fn register(&self, username: &str, password_str: &str) -> Result<JwtResponse> {
        if self.repo.find_by_username(username).await?.is_some() {
            bail!("Username already taken");
        }

        let password_hash = password::hash_password(password_str)?;
        let user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            password_hash,
            telegram_id: None,
        };

        self.repo.create(&user).await?;
        let tokens = self.jwt.generate_tokens(user.id);
        let response = JwtResponse {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
        };
        Ok(response)
    }

    pub async fn login(&self, username: &str, password_str: &str) -> Result<JwtResponse> {
        let user = self
            .repo
            .find_by_username(username)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid username or password"))?;

        let ok = password::verify_password(&user.password_hash, password_str)?;
        if !ok {
            bail!("Invalid username or password");
        }

        let tokens = self.jwt.generate_tokens(user.id);
        let response = JwtResponse {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
        };
        Ok(response)
    }

    pub async fn login_telegram(&self, dto: &LoginTelegramBotDto) -> Result<JwtResponse> {
        // 💡 ты можешь сверять client_id, secret, telegram_id как хочешь
        let username = &dto.username;
        let telegram_id = &dto.telegram_id;
        let maybe_user = self.repo.find_by_username(username).await?;

        let user = if let Some(user) = maybe_user {
            user
        } else {
            // создаём нового пользователя
            let new_user = User {
                id: Uuid::new_v4(),
                username: username.clone(),
                password_hash: "telegram".into(), // ⚠️ фейковый пароль
                telegram_id: Some(telegram_id.clone()),
            };
            self.repo.create(&new_user).await?;
            new_user
        };

        let tokens = self.jwt.generate_tokens(user.id);
        let response = JwtResponse {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
        };
        Ok(response)
    }
}
