use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::application::dtos::pagination_dto::Pagination;
use crate::application::traits::transaction_repo::TransactionRepository;
use crate::domain::transaction::{CreateTransaction, Transaction, TransactionType, UpdateTransaction};

#[derive(Clone)]
pub struct PostgresTransactionRepo {
    pub pool: PgPool,
}

#[async_trait]
impl TransactionRepository for PostgresTransactionRepo {
    async fn save(&self, transaction: CreateTransaction) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO accounting_transactions (
                id,
                user_id,
                category_id,
                amount,
                description,
                created_at,
                type
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            transaction.id,
            transaction.user_id,
            transaction.category_id,
            transaction.amount,
            transaction.description,
            transaction.created_at,
            match transaction.transaction_type {
                TransactionType::Income => "INCOME",
                TransactionType::Expense => "EXPENSE",
            }
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    
    async fn find_by_id_and_user_id(&self, id: Uuid, user_id: Uuid) -> anyhow::Result<Option<Transaction>> {
        let row = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                transaction.id,
                transaction.user_id,
                categories.code as category_code,
                transaction.amount,
                transaction.description,
                transaction.created_at,
                transaction.type as transaction_type
            from accounting_transactions transaction
            join accounting_categories as categories on categories.id = transaction.category_id
            where transaction.id = $1 and transaction.user_id = $2
            "#,
            id,
            user_id
        ).fetch_optional(&self.pool).await?;
        
        Ok(row)
    }

    async fn find_all(&self, user_id: Uuid, pagination: &Pagination) -> anyhow::Result<Vec<Transaction>> {
        let rows = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                transaction.id,
                transaction.user_id,
                categories.code as category_code,
                transaction.amount,
                transaction.description,
                transaction.created_at,
                transaction.type as transaction_type
            FROM accounting_transactions transaction
            JOIN accounting_categories as categories ON categories.id = transaction.category_id
            WHERE transaction.user_id = $1
            ORDER BY transaction.created_at, transaction.category_id
            LIMIT $2 OFFSET $3
            "#,
            user_id,
            pagination.size,
            pagination.offset()
        )
            .fetch_all(&self.pool)
            .await?;
        
        Ok(rows)
    }

    async fn count(&self, user_id: Uuid) -> anyhow::Result<i64> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*)
            FROM accounting_transactions
            WHERE user_id = $1
            "#,
            user_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(result.count.unwrap())
    }

    async fn delete(&self, user_id: Uuid, id: Uuid) -> anyhow::Result<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM accounting_transactions
            WHERE user_id = $1 and id = $2
            "#,
            user_id,
            id
        )
            .execute(&self.pool)
            .await?;
        
        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Transaction with id '{}' not found", id));
        }

        Ok(())
    }

    async fn find_all_by_month(&self, user_id: Uuid, year: u32, month: u32, pagination: &Pagination) -> anyhow::Result<Vec<Transaction>> {
        let rows = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                transaction.id,
                transaction.user_id,
                categories.code as category_code,
                transaction.amount,
                transaction.description,
                transaction.created_at,
                transaction.type as transaction_type
            FROM accounting_transactions transaction
            JOIN accounting_categories as categories ON categories.id = transaction.category_id
            WHERE
                transaction.user_id = $1
                AND EXTRACT(YEAR FROM transaction.created_at)::INTEGER = $2
                AND EXTRACT(MONTH FROM transaction.created_at)::INTEGER = $3
            ORDER BY transaction.created_at, transaction.category_id
            LIMIT $4 OFFSET $5
            "#,
            user_id,
            year as i32,
            month as i32,
            pagination.size,
            pagination.offset()
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    async fn update(&self, user_id: Uuid, transaction_id: Uuid, transaction: UpdateTransaction) -> anyhow::Result<()> {
        let result = sqlx::query!(
            r#"
            UPDATE accounting_transactions
            SET
                category_id = $1,
                amount = $2,
                description = $3,
                created_at = $4
            WHERE
                    user_id = $5
                and id = $6
            "#,
            transaction.category_id,
            transaction.amount,
            transaction.description,
            transaction.created_at,
            user_id,
            transaction_id
        ).execute(&self.pool).await?;
        
        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Transaction with id '{}' not found", transaction_id));
        }
        
        Ok(())
    }
}
