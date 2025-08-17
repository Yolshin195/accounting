use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::dtos::pagination_dto::Pagination;

#[derive(Serialize, Deserialize)]
pub struct TransactionDto {
    pub id: Uuid,
    #[serde(with = "rust_decimal::serde::float")]
    pub amount: Decimal,
    #[serde(rename = "category")]
    pub category_code: String,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    #[serde(rename = "type")]
    pub transaction_type: String, // "INCOME" or "EXPENSE"
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateTransactionDto {
    pub amount: Decimal,
    #[serde(rename = "category")]
    pub category_code: String,
    pub description: Option<String>,
    pub date: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct MonthlyTransactionQuery {
    pub year: Option<u32>,
    pub month: Option<u32>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

