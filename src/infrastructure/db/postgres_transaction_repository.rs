use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::application::dtos::pagination_dto::Pagination;
use crate::application::traits::transaction_repo::TransactionRepository;
use crate::domain::transaction::Transaction;

#[derive(Clone)]
pub struct PostgresTransactionRepo {
    pub pool: PgPool,
}

#[async_trait]
impl TransactionRepository for PostgresTransactionRepo {
    async fn save(&self, transaction: Transaction) -> anyhow::Result<Transaction> {
        todo!()
    }

    async fn find_all(&self, user_id: Uuid, pagination: Pagination) -> anyhow::Result<Vec<Transaction>> {
        todo!()
    }
}