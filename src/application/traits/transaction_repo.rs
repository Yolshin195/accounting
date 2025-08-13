use crate::domain::transaction::Transaction;
use async_trait::async_trait;
use uuid::Uuid;
use crate::application::dtos::pagination_dto::Pagination;

#[async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn save(&self, transaction: Transaction) -> anyhow::Result<Transaction>;
    async fn find_all(&self, user_id: Uuid, pagination: Pagination) -> anyhow::Result<Vec<Transaction>>;
}