use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;
use crate::application::dtos::pagination_dto::{PagedResponse, Pagination};
use crate::application::dtos::transaction_dto::{CreateTransactionDto, TransactionDto};
use crate::application::traits::category_repo::CategoryRepository;
use crate::application::traits::transaction_repo::TransactionRepository;
use crate::domain::transaction::{CreateTransaction, TransactionType};

#[derive(Clone)]
pub struct TransactionService<R: TransactionRepository, CR: CategoryRepository> {
    pub repo: R,
    pub category_repo: CR,
}

impl<R: TransactionRepository, CR: CategoryRepository> TransactionService<R, CR> {
    pub fn new(repo: R, category_repo: CR) -> Self {
        Self { repo, category_repo }
    }
    
    pub async fn create_expense(&self, user_id: Uuid, transaction: CreateTransactionDto) -> anyhow::Result<TransactionDto> {
        self.create(user_id, transaction, TransactionType::Expense).await
    }
    
    pub async fn create_income(&self, user_id: Uuid, transaction: CreateTransactionDto) -> anyhow::Result<TransactionDto> {
        self.create(user_id, transaction, TransactionType::Income).await
    }
    
    async fn create(&self, user_id: Uuid, transaction: CreateTransactionDto, transaction_type: TransactionType) -> anyhow::Result<TransactionDto> {
        let category = self.category_repo.find_by_code(
            user_id, transaction.category_code
        ).await?;

        if category.is_none() {
            return Err(anyhow::anyhow!("Category not found"));
        }

        let new_transaction = CreateTransaction {
            id: Uuid::new_v4(),
            user_id,
            amount: transaction.amount,
            category_id: category.unwrap().id,
            description: transaction.description,
            created_at: Utc::now(),
            transaction_type
        };

        let created = self.repo.save(new_transaction).await?;

        Ok(TransactionDto {
            id: created.id,
            amount: created.amount,
            category_code: created.category_code,
            description: created.description,
            date: created.created_at,
            transaction_type: created.transaction_type.to_string().to_uppercase()
        })
    }

    pub async fn get_all(&self, user_id: Uuid, pagination: Pagination) -> anyhow::Result<PagedResponse<TransactionDto>> {
        let count = self.repo.count(user_id).await?;
        let list = self.repo.find_all(user_id, &pagination).await?;
        let list_dto = list.into_iter()
            .map(|row| TransactionDto {
                id: row.id,
                amount: row.amount,
                category_code: row.category_code,
                description: row.description,
                date: row.created_at,
                transaction_type: row.transaction_type.to_string().to_uppercase()
            })
            .collect();
        let response = PagedResponse::new(list_dto, &pagination, count);
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use rust_decimal::Decimal;
    use crate::application::traits::category_repo::mock::MockInMemoryCategoryRepository;
    use crate::application::traits::transaction_repo::mock::InMemoryTransactionRepo;
    use crate::domain::transaction::{TransactionType};
    use super::*;
    
    #[tokio::test]
    async fn test_gata_all() {
        // Given
        let user_id = Uuid::new_v4();
        let pagination = Pagination::default();
        let transaction = CreateTransaction {
            id: Uuid::new_v4(),
            user_id,
            category_id: Uuid::new_v4(),
            amount: Decimal::new(100, 0),
            description: Some("FOOD category".to_string()),
            created_at: Utc::now(),
            transaction_type: TransactionType::Expense
        };
        let repo = InMemoryTransactionRepo::new();
        let category_repo = MockInMemoryCategoryRepository::new();
        repo.save(transaction.clone()).await.unwrap();
        let service = TransactionService::new(repo, category_repo);
        let page_res = service.get_all(user_id, pagination).await.unwrap();
        
        assert_eq!(page_res.page.total_elements, 1);
        assert_eq!(page_res.page.total_pages, 1);
        assert_eq!(page_res.content.len(), 1);
    }
}