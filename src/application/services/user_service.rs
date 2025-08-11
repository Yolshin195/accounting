use crate::application::traits::user_repo::UserRepository;
use crate::domain::user::User;
use crate::infrastructure::auth::password::{hash_password, verify_password};

use anyhow::{Result, bail};
use uuid::Uuid;

pub struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// Регистрация нового пользователя
    pub async fn register(&self, username: &str, password: &str) -> Result<User> {
        if self.repo.find_by_username(username).await?.is_some() {
            bail!("Username '{}' is already taken", username);
        }

        let password_hash = hash_password(password)?;

        let user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            password_hash,
            telegram_id: None,
        };

        self.repo.create(&user).await?;
        Ok(user)
    }

    /// Проверка логина и пароля
    pub async fn authenticate(&self, username: &str, password: &str) -> Result<Option<User>> {
        let user = match self.repo.find_by_username(username).await? {
            Some(user) => {
                if verify_password(&user.password_hash, password)? {
                    Some(user)
                } else {
                    None
                }
            }
            None => None,
        };

        Ok(user)
    }
}
