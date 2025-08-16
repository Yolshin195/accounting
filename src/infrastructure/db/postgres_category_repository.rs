use crate::application::traits::category_repo::CategoryRepository;
use crate::domain::category::{Category, CategoryType};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::application::dtos::pagination_dto::Pagination;

#[derive(Clone)]
pub struct PostgresCategoryRepo {
    pub pool: PgPool,
}

#[async_trait]
impl CategoryRepository for PostgresCategoryRepo {
    async fn save(&self, category: Category) -> anyhow::Result<Category> {
        sqlx::query!(
            r#"
            INSERT INTO accounting_categories (id, user_id, code, name, description, type)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            category.id,
            category.user_id,
            category.code,
            category.name,
            category.description,
            match category.category_type {
                CategoryType::Income => "INCOME",
                CategoryType::Expense => "EXPENSE",
            }
        )
        .execute(&self.pool)
        .await?;

        Ok(category)
    }

    async fn find_all(&self, user_id: Uuid, pagination: Pagination) -> anyhow::Result<Vec<Category>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, code, name, description, type
            FROM accounting_categories
            WHERE user_id = $1
            ORDER BY code
            LIMIT $2 OFFSET $3
            "#,
            user_id,
            pagination.size,
            pagination.offset()
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Category {
                id: row.id,
                user_id: row.user_id,
                code: row.code,
                name: row.name,
                description: row.description,
                category_type: CategoryType::from_str(row.r#type.as_str()).unwrap()
            })
            .collect())
    }
    
    async fn delete_by_code(&self, user_id: Uuid, code: String) -> anyhow::Result<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM accounting_categories
            WHERE user_id = $1 AND code = $2
            "#,
            user_id,
            code
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Category with code '{}' not found", code));
        }
        
        Ok(())
    }

    async fn count(&self, user_id: Uuid) -> anyhow::Result<i64> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*)
            FROM accounting_categories
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(result.count.unwrap())
    }

    async fn find_by_code(&self, user_id: Uuid, code: String) -> anyhow::Result<Option<Category>> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, code, name, description, type
            FROM accounting_categories
            WHERE user_id = $1 AND code = $2
            "#,
            user_id,
            code
        ).fetch_one(&self.pool).await?;

        Ok(Some(Category {
            id: row.id,
            user_id: row.user_id,
            code: row.code,
            name: row.name,
            description: row.description,
            category_type: CategoryType::from_str(row.r#type.as_str()).unwrap()
        }))
    }
}
