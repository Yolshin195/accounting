use crate::application::dtos::pagination_dto::Pagination;
use crate::application::traits::currency_repo::CurrencyRepository;
use crate::application::traits::reference_repo::{ReferenceFilter, ReferenceRepository};
use crate::domain::currency::Currency;
use crate::domain::reference::ReferenceError;
use crate::infrastructure::db::reference_queries;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

const TABLE: &str = "accounting_currencies";
const SELECT: &str = r#"
    SELECT t.id, t.user_id, t.code, t.name, t.description, t.sort_order, t.is_archived,
           t.symbol, t.decimals, t.is_base
    FROM accounting_currencies t"#;

#[derive(Clone)]
pub struct PostgresCurrencyRepository {
    pub pool: PgPool,
}

#[async_trait]
impl ReferenceRepository<Currency> for PostgresCurrencyRepository {
    async fn find_all(&self, user_id: Uuid, filter: &ReferenceFilter, pagination: &Pagination) -> Result<Vec<Currency>, ReferenceError> {
        reference_queries::find_all(&self.pool, SELECT, user_id, filter, pagination).await
    }

    async fn count(&self, user_id: Uuid, filter: &ReferenceFilter) -> Result<i64, ReferenceError> {
        reference_queries::count(&self.pool, TABLE, user_id, filter).await
    }

    async fn find_by_code(&self, user_id: Uuid, code: &str) -> Result<Option<Currency>, ReferenceError> {
        reference_queries::find_by_code(&self.pool, SELECT, user_id, code).await
    }

    async fn find_active_id_by_code(&self, user_id: Uuid, code: &str) -> Result<Option<Uuid>, ReferenceError> {
        reference_queries::find_active_id_by_code(&self.pool, TABLE, user_id, code).await
    }

    async fn set_archived(&self, user_id: Uuid, code: &str, archived: bool) -> Result<bool, ReferenceError> {
        reference_queries::set_archived(&self.pool, TABLE, user_id, code, archived).await
    }
}

#[async_trait]
impl CurrencyRepository for PostgresCurrencyRepository {
    async fn insert(&self, c: &Currency) -> Result<(), ReferenceError> {
        sqlx::query!(
            r#"
            INSERT INTO accounting_currencies
                (id, user_id, code, name, description, sort_order, symbol, decimals, is_base)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            c.base.id,
            c.base.user_id,
            c.base.code,
            c.base.name,
            c.base.description,
            c.base.sort_order,
            c.symbol,
            c.decimals,
            c.is_base
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, c: &Currency) -> Result<(), ReferenceError> {
        sqlx::query!(
            r#"
            UPDATE accounting_currencies
            SET name = $3, description = $4, sort_order = $5, symbol = $6, decimals = $7, is_base = $8
            WHERE id = $1 AND user_id = $2
            "#,
            c.base.id,
            c.base.user_id,
            c.base.name,
            c.base.description,
            c.base.sort_order,
            c.symbol,
            c.decimals,
            c.is_base
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
