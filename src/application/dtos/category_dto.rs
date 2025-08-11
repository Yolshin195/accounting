use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateCategoryDto {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub category_type: String, // "INCOME" or "EXPENSE"
}

#[derive(Serialize, Deserialize)]
pub struct CategoryDto {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub category_type: String,
}
