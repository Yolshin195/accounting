use chrono::{Datelike, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use crate::application::dtos::pagination_dto::{PagedResponse, Pagination};
use crate::application::dtos::transaction_dto::{CategoryExpenseSummaryDto, CreateTransactionDto, TransactionDto, UpdateTransactionDto};
use crate::application::traits::category_repo::CategoryRepository;
use crate::application::traits::transaction_repo::TransactionRepository;
use crate::domain::transaction::{CreateTransaction, TransactionType, UpdateTransaction};

#[derive(Clone)]
pub struct TransactionService<TR: TransactionRepository, CR: CategoryRepository> {
    pub repo: TR,
    pub category_repo: CR,
}

impl<TR: TransactionRepository, CR: CategoryRepository> TransactionService<TR, CR> {
    pub fn new(repo: TR, category_repo: CR) -> Self {
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

        let transaction_id = Uuid::new_v4();
        let new_transaction = CreateTransaction {
            id: transaction_id,
            user_id,
            amount: transaction.amount,
            category_id: category.unwrap().id,
            description: transaction.description,
            created_at: match transaction.date{
                Some(date) => date.and_hms_micro_opt(0, 0, 0, 0).unwrap(),
                None => Utc::now().naive_utc()
            },
            transaction_type
        };

        self.repo.save(new_transaction).await?;

        let created = self.repo.find_by_id_and_user_id(transaction_id, user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction not found"))?;

        Ok(TransactionDto {
            id: created.id,
            amount: created.amount,
            category_code: created.category_code,
            description: created.description,
            date: created.created_at.and_utc(),
            transaction_type: created.transaction_type.to_string().to_uppercase()
        })
    }

    pub async fn update(&self, user_id: Uuid, id: Uuid, transaction: UpdateTransactionDto) -> anyhow::Result<TransactionDto> {
        let category = self.category_repo.find_by_code(
            user_id, transaction.category_code
        ).await?;

        if category.is_none() {
            return Err(anyhow::anyhow!("Category not found"));
        }

        let data_for_update = UpdateTransaction {
            category_id: category.unwrap().id,
            amount: transaction.amount,
            description: transaction.description,
            created_at: transaction.date.naive_utc()
        };

        self.repo.update(user_id, id, data_for_update).await?;

        let response = self.repo.find_by_id_and_user_id(id, user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction not found"))?;

        Ok(
            TransactionDto {
                id: response.id,
                amount: response.amount,
                category_code: response.category_code,
                description: response.description,
                date: response.created_at.and_utc(),
                transaction_type: response.transaction_type.to_string().to_uppercase()
            }
        )
    }

    pub async fn get_all(&self, user_id: Uuid, pagination: Pagination) -> anyhow::Result<PagedResponse<TransactionDto>> {
        let total_elements = self.repo.count(user_id).await?;
        let list = self.repo.find_all(user_id, &pagination).await?;
        let list_dto = list.into_iter()
            .map(|row| TransactionDto {
                id: row.id,
                amount: row.amount,
                category_code: row.category_code,
                description: row.description,
                date: row.created_at.and_utc(),
                transaction_type: row.transaction_type.to_string().to_uppercase()
            })
            .collect();
        let response = PagedResponse::new(list_dto, &pagination, total_elements);
        Ok(response)
    }

    pub async fn find_by_user_id_and_id(&self, user_id: Uuid, id: Uuid) -> anyhow::Result<TransactionDto> {
        let response = self.repo.find_by_id_and_user_id(id, user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction not found"))?;
        Ok(
            TransactionDto {
                id: response.id,
                amount: response.amount,
                category_code: response.category_code,
                description: response.description,
                date: response.created_at.and_utc(),
                transaction_type: response.transaction_type.to_string().to_uppercase()
            }
        )
    }

    pub async fn find_all_by_month(&self, user_id: Uuid, year: Option<u32>, month: Option<u32>, pagination: Pagination) -> anyhow::Result<PagedResponse<TransactionDto>> {
        let total_elements = 1000;
        let year = year.unwrap_or_else(|| Utc::now().year() as u32);
        let month = month.unwrap_or_else(|| Utc::now().month());
        let list = self.repo.find_all_by_month(user_id, year, month, &pagination).await?;
        let list_dto = list.into_iter()
            .map(|row| TransactionDto {
                id: row.id,
                amount: row.amount,
                category_code: row.category_code,
                description: row.description,
                date: row.created_at.and_utc(),
                transaction_type: row.transaction_type.to_string().to_uppercase()
            })
            .collect();
        let response = PagedResponse::new(list_dto, &pagination, total_elements);
        Ok(response)
    }

    pub async fn delete(&self, user_id: Uuid, id: Uuid) -> anyhow::Result<()> {
        self.repo.delete(user_id, id).await
    }
    
    pub async fn sum_today_expenses_grouped_by_category(&self, user_id: Uuid) -> anyhow::Result<Vec<CategoryExpenseSummaryDto>> {
        let today: NaiveDate = Utc::now().date_naive();
        self.repo.sum_today_expenses_grouped_by_category(user_id, today).await
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

    fn get_mock_transaction() -> CreateTransaction {
        CreateTransaction {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            category_id: Uuid::new_v4(),
            amount: Decimal::new(100, 0),
            description: Some("FOOD category".to_string()),
            created_at: Utc::now().naive_utc(),
            transaction_type: TransactionType::Expense
        }
    }

    fn get_service() -> TransactionService<InMemoryTransactionRepo, MockInMemoryCategoryRepository> {
        let repo = InMemoryTransactionRepo::new();
        let category_repo = MockInMemoryCategoryRepository::new();
        TransactionService::new(repo, category_repo)
    }
    
    #[tokio::test]
    async fn test_gata_all() {
        // Given
        let pagination = Pagination::default();
        let transaction = get_mock_transaction();
        let repo = InMemoryTransactionRepo::new();
        let category_repo = MockInMemoryCategoryRepository::new();
        repo.save(transaction.clone()).await.unwrap();
        let service = TransactionService::new(repo, category_repo);
        let page_res = service.get_all(transaction.user_id, pagination).await.unwrap();
        
        assert_eq!(page_res.page.total_elements, 1);
        assert_eq!(page_res.page.total_pages, 1);
        assert_eq!(page_res.content.len(), 1);
    }

    #[tokio::test]
    async fn test_create_expense() {
        let user_id = Uuid::new_v4();
        let service = get_service();
        let create_transaction_dto = CreateTransactionDto {
            amount: Decimal::new(100, 0),
            category_code: "FOOD".to_string(),
            description: Some("FOOD category".to_string()),
            date: None,
        };
        let created_transaction = service.create_expense(user_id, create_transaction_dto.clone()).await.unwrap();

        assert_eq!(created_transaction.amount, create_transaction_dto.amount);
        assert_eq!(created_transaction.category_code, create_transaction_dto.category_code);
        assert_eq!(created_transaction.description.unwrap(), create_transaction_dto.description.unwrap());
        assert_eq!(created_transaction.transaction_type.to_string(), "EXPENSE".to_string());
    }

    #[tokio::test]
    async fn test_create_income() {
        let user_id = Uuid::new_v4();
        let service = get_service();
        let create_transaction_dto = CreateTransactionDto {
            amount: Decimal::new(100, 0),
            category_code: "FOOD".to_string(),
            description: Some("FOOD category".to_string()),
            date: None,
        };
        let created_transaction = service.create_income(user_id, create_transaction_dto.clone()).await.unwrap();

        assert_eq!(created_transaction.amount, create_transaction_dto.amount);
        assert_eq!(created_transaction.category_code, create_transaction_dto.category_code);
        assert_eq!(created_transaction.description.unwrap(), create_transaction_dto.description.unwrap());
        assert_eq!(created_transaction.transaction_type, "INCOME".to_string());
    }
}