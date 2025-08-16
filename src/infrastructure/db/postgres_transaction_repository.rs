use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::application::dtos::pagination_dto::Pagination;
use crate::application::traits::transaction_repo::TransactionRepository;
use crate::domain::transaction::{CreateTransaction, Transaction};

#[derive(Clone)]
pub struct PostgresTransactionRepo {
    pub pool: PgPool,
}

#[async_trait]
impl TransactionRepository for PostgresTransactionRepo {
    async fn save(&self, transaction: CreateTransaction) -> anyhow::Result<Transaction> {
        todo!()
    }

    async fn find_all(&self, user_id: Uuid, pagination: &Pagination) -> anyhow::Result<Vec<Transaction>> {
        todo!()
    }

    async fn count(&self, user_id: Uuid) -> anyhow::Result<i64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_all() {
        
    }
}