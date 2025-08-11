use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::category::{Category, CategoryType};
use crate::application::traits::category_repo::CategoryRepository;
use uuid::Uuid;

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

    async fn find_all(&self, user_id: Uuid) -> anyhow::Result<Vec<Category>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, code, name, description, type
            FROM accounting_categories
            WHERE user_id = $1
            "#,
            user_id
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
                category_type: if row.r#type == "INCOME" {
                    CategoryType::Income
                } else {
                    CategoryType::Expense
                },
            })
            .collect())
    }
}
