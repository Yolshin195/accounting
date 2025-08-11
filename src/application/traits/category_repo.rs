use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::category::Category;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn save(&self, category: Category) -> anyhow::Result<Category>;
    async fn find_all(&self, user_id: Uuid) -> anyhow::Result<Vec<Category>>;
}
