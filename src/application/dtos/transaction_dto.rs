use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TransactionDto {
    pub id: String,
}
