use crate::domain::category::CategoryType;
use std::fmt;
use chrono::{NaiveDateTime};
use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionType {
    Expense,
    Income,
}

impl std::str::FromStr for TransactionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "expense" => Ok(Self::Expense),
            "income" => Ok(Self::Income),
            _ => Err(format!("Invalid transaction type: {}", s)),
        }
    }
}

impl From<String> for TransactionType {
    fn from(s: String) -> Self {
        s.as_str().parse().unwrap_or(Self::Expense) // По умолчанию Expense если не удалось распарсить
    }
}


impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expense => write!(f, "Expense"),
            Self::Income => write!(f, "Income"),
        }
    }
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
pub struct CreateTransaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub amount: Decimal,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub transaction_type: TransactionType,
}

#[derive(Debug, Clone)]
pub struct UpdateTransaction {
    pub category_id: Uuid,
    pub amount: Decimal,
    pub description: Option<String>,
    pub created_at: NaiveDateTime
}

#[derive(Debug, Clone, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_code: String,
    pub amount: Decimal,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub transaction_type: TransactionType,
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transaction_type_to_string() {
        let t = TransactionType::Expense;
        assert_eq!(t.to_string(), "Expense");
    }
}
