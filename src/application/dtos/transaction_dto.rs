use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct TransactionDto {
    pub id: Uuid,
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
