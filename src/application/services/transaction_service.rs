use uuid::Uuid;
use crate::application::dtos::pagination_dto::{PagedResponse, Pagination};
use crate::application::dtos::transaction_dto::TransactionDto;
use crate::application::traits::transaction_repo::TransactionRepository;

#[derive(Clone)]
pub struct TransactionService<R: TransactionRepository> {
    pub repo: R,
}

impl<R: TransactionRepository> TransactionService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self, user_id: Uuid, pagination: Pagination) -> anyhow::Result<PagedResponse<TransactionDto>> {
        Ok(PagedResponse::new(vec![], &pagination, 0))
    }
}