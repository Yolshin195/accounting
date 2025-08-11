use crate::domain::category::Category;
use async_trait::async_trait;
use uuid::Uuid;
use crate::application::dtos::pagination_dto::Pagination;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn save(&self, category: Category) -> anyhow::Result<Category>;
    async fn find_all(&self, user_id: Uuid, pagination: Pagination) -> anyhow::Result<Vec<Category>>;
}
