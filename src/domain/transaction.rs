use crate::domain::category::CategoryType;
use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionType {
    Expense,
    Income,
}

impl TransactionType {
    pub fn from_category_type(cat_type: &CategoryType) -> Self {
        match cat_type {
            CategoryType::Expense => Self::Expense,
            CategoryType::Income => Self::Income,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "EXPENSE" => Some(Self::Expense),
            "INCOME" => Some(Self::Income),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub amount: Decimal,
    pub category_code: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub transaction_type: TransactionType,
}
