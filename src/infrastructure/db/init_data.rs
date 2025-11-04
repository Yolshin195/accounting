use crate::infrastructure::auth::password;
use std::env;
use sqlx::PgPool;


pub async fn create_telegram_client_user(pool: &PgPool) -> anyhow::Result<(), anyhow::Error> {
    let client_id = env::var("TELEGRAM_BOT_CLIENT_ID")
        .expect("TELEGRAM_BOT_CLIENT_ID must be set");
    let secret = env::var("TELEGRAM_BOT_SECRET")
        .expect("TELEGRAM_BOT_SECRET must be set");

    sqlx::query!(
            r#"
            INSERT INTO accounting_users (id, telegram_id, username, password_hash, is_system)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (username) DO NOTHING
            "#,
            uuid::Uuid::new_v4(),
            Option::<String>::None,
            client_id,
            password::hash_password(&secret)?,
            true,
        )
        .execute(pool)
        .await?;
    Ok(())
}