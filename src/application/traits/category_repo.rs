use crate::domain::category::Category;
use async_trait::async_trait;
use uuid::Uuid;
use crate::application::dtos::pagination_dto::Pagination;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn save(&self, category: Category) -> anyhow::Result<Category>;
    async fn find_all(&self, user_id: Uuid, pagination: Pagination) -> anyhow::Result<Vec<Category>>;
    async fn delete_by_code(&self, user_id: Uuid, code: String) -> anyhow::Result<()>;
    async fn count(&self, user_id: Uuid) -> anyhow::Result<i64>;
    async fn find_by_code(&self, user_id: Uuid, code: String) -> anyhow::Result<Option<Category>>;
}


#[cfg(test)]
pub mod mock {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use crate::domain::category::CategoryType;
    // ToDo: Использовать асинхронный Mutex
    use super::*;

    #[derive(Clone)]
    pub struct MockInMemoryCategoryRepository{
        rows : Arc<Mutex<HashMap<Uuid, HashMap<Uuid, Category>>>>
    }

    impl MockInMemoryCategoryRepository{
        pub fn new() -> Self {
            Self {
                rows: Arc::new(Mutex::new(HashMap::new()))
            }
        }
    }

    #[async_trait]
    impl CategoryRepository for MockInMemoryCategoryRepository {
        async fn save(&self, category: Category) -> anyhow::Result<Category> {
            {
                let mut rows = self.rows.lock()
                    .map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;

                // Получаем или создаем HashMap для пользователя
                let user_transactions = rows.entry(category.user_id).or_insert_with(HashMap::new);

                // Сохраняем транзакцию
                user_transactions.insert(category.id, category.clone());
            }

            Ok(category)
        }

        async fn find_all(&self, user_id: Uuid, pagination: Pagination) -> anyhow::Result<Vec<Category>> {
            todo!()
        }

        async fn delete_by_code(&self, user_id: Uuid, code: String) -> anyhow::Result<()> {
            todo!()
        }

        async fn count(&self, user_id: Uuid) -> anyhow::Result<i64> {
            todo!()
        }

        async fn find_by_code(&self, user_id: Uuid, code: String) -> anyhow::Result<Option<Category>> {
            let rows = self.rows.lock()
                .map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;

            if let Some(user_categories) = rows.get(&user_id) {
                Ok(user_categories.values().find(|category| category.code == code).cloned())
            } else {
                Ok(None)
            }
        }
    }
    
    #[tokio::test]
    async fn test_find_by_code() -> anyhow::Result<()> {
        // Given
        let user_id = Uuid::new_v4();
        let code: &str = "FOOD";
        let category = Category {
            id: Uuid::new_v4(),
            user_id,
            code: code.to_string(),
            name: "Еда".to_string(),
            description: Some("Тестовая категория едв".to_string()),
            category_type: CategoryType::Expense
        };
        let repo = MockInMemoryCategoryRepository::new();
        _ = repo.save(category).await?;
        let find_category = repo.find_by_code(user_id, code.to_string()).await?;
        
        assert_eq!(find_category.unwrap().code, code.to_string());
        Ok(())
    }
}
