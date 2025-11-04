use crate::application::traits::user_repo::UserRepository;
use crate::domain::user::User;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pub pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
            INSERT INTO accounting_users (id, telegram_id, username, password_hash, is_system)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user.id,
            user.telegram_id,
            user.username,
            user.password_hash,
            user.is_system,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, anyhow::Error> {
        let record = sqlx::query!(
            r#"
            SELECT id, telegram_id, username, password_hash, is_system
            FROM accounting_users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|row| User {
            id: row.id,
            telegram_id: row.telegram_id,
            username: row.username,
            password_hash: row.password_hash,
            is_system: row.is_system,
        }))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, anyhow::Error> {
        let record = sqlx::query!(
            r#"
            SELECT id, telegram_id, username, password_hash, is_system
            FROM accounting_users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|row| User {
            id: row.id,
            telegram_id: row.telegram_id,
            username: row.username,
            password_hash: row.password_hash,
            is_system: row.is_system,
        }))
    }
    
    async fn find_by_telegram_id(&self, telegram_id: &String) -> Result<Option<User>, anyhow::Error> {
        let record = sqlx::query!(
            r#"
            SELECT id, telegram_id, username, password_hash, is_system
            FROM accounting_users
            WHERE telegram_id = $1
            "#,
            telegram_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(record.map(|row| User {
            id: row.id,
            telegram_id: row.telegram_id,
            username: row.username,
            password_hash: row.password_hash,
            is_system: row.is_system,
        }))
    }
}
