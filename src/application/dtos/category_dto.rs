use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateCategoryDto {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub category_type: String, // "INCOME" or "EXPENSE"
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CategoryDto {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub category_type: String,
}
