use sqlx::{PgPool, postgres::PgPoolOptions, migrate::Migrator};
use std::env;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub async fn init_pg_pool() -> anyhow::Result<PgPool> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    MIGRATOR.run(&pool).await?;

    Ok(pool)
}
