use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CategoryType {
    Expense,
    Income,
}

impl CategoryType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "EXPENSE" => Some(Self::Expense),
            "INCOME" => Some(Self::Income),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Category {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub category_type: CategoryType,
}