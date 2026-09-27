use crate::application::dtos::pagination_dto::Pagination;
use crate::application::traits::account_repo::AccountRepository;
use crate::application::traits::reference_repo::{ReferenceFilter, ReferenceRepository};
use crate::domain::account::Account;
use crate::domain::reference::ReferenceError;
use crate::infrastructure::db::reference_queries;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

const TABLE: &str = "accounting_accounts";
const SELECT: &str = r#"
    SELECT t.id, t.user_id, t.code, t.name, t.description, t.sort_order, t.is_archived,
           t.currency_id, c.code AS currency_code, t.account_type, t.initial_balance
    FROM accounting_accounts t
    JOIN accounting_currencies c ON c.id = t.currency_id"#;

#[derive(Clone)]
pub struct PostgresAccountRepository {
    pub pool: PgPool,
}

#[async_trait]
impl ReferenceRepository<Account> for PostgresAccountRepository {
    async fn find_all(&self, user_id: Uuid, filter: &ReferenceFilter, pagination: &Pagination) -> Result<Vec<Account>, ReferenceError> {
        reference_queries::find_all(&self.pool, SELECT, user_id, filter, pagination).await
    }

    async fn count(&self, user_id: Uuid, filter: &ReferenceFilter) -> Result<i64, ReferenceError> {
        reference_queries::count(&self.pool, TABLE, user_id, filter).await
    }

    async fn find_by_code(&self, user_id: Uuid, code: &str) -> Result<Option<Account>, ReferenceError> {
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
impl AccountRepository for PostgresAccountRepository {
    async fn insert(&self, a: &Account) -> Result<(), ReferenceError> {
        sqlx::query!(
            r#"
            INSERT INTO accounting_accounts
                (id, user_id, code, name, description, sort_order, currency_id, account_type, initial_balance)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            a.base.id,
            a.base.user_id,
            a.base.code,
            a.base.name,
            a.base.description,
            a.base.sort_order,
            a.currency_id,
            a.account_type.as_str(),
            a.initial_balance
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, a: &Account) -> Result<(), ReferenceError> {
        sqlx::query!(
            r#"
            UPDATE accounting_accounts
            SET name = $3, description = $4, sort_order = $5,
                currency_id = $6, account_type = $7, initial_balance = $8
            WHERE id = $1 AND user_id = $2
            "#,
            a.base.id,
            a.base.user_id,
            a.base.name,
            a.base.description,
            a.base.sort_order,
            a.currency_id,
            a.account_type.as_str(),
            a.initial_balance
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
