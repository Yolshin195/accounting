use crate::application::dtos::category_dto::{CategoryDto, CreateCategoryDto};
use crate::application::traits::category_repo::CategoryRepository;
use crate::domain::category::{Category, CategoryType};
use uuid::Uuid;
use crate::application::dtos::pagination_dto::Pagination;

#[derive(Clone)]
pub struct CategoryService<R: CategoryRepository> {
    pub repo: R,
}

impl<R: CategoryRepository> CategoryService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create(
        &self,
        dto: CreateCategoryDto,
        user_id: Uuid,
    ) -> anyhow::Result<CategoryDto> {
        let category = Category {
            id: Uuid::new_v4(),
            user_id,
            code: dto.code,
            name: dto.name,
            description: dto.description,
            category_type: match dto.category_type.as_str() {
                "INCOME" => CategoryType::Income,
                "EXPENSE" => CategoryType::Expense,
                _ => return Err(anyhow::anyhow!("Invalid category type")),
            },
        };

        let created = self.repo.save(category.clone()).await?;

        Ok(CategoryDto {
            id: created.id.to_string(),
            code: created.code,
            name: created.name,
            description: created.description,
            category_type: match created.category_type {
                CategoryType::Income => "INCOME".into(),
                CategoryType::Expense => "EXPENSE".into(),
            },
        })
    }

    pub async fn get_all(&self, user_id: Uuid, pagination: Pagination) -> anyhow::Result<Vec<CategoryDto>> {
        let list = self.repo.find_all(user_id, pagination).await?;
        Ok(list
            .into_iter()
            .map(|cat| CategoryDto {
                id: cat.id.to_string(),
                code: cat.code,
                name: cat.name,
                description: cat.description,
                category_type: match cat.category_type {
                    CategoryType::Income => "INCOME".into(),
                    CategoryType::Expense => "EXPENSE".into(),
                },
            })
            .collect())
    }
}
