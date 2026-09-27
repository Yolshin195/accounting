use crate::domain::reference::ReferenceBase;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountType {
    /// Наличные
    Cash,
    /// Карта
    Card,
    /// Депозит
    Deposit,
}

impl AccountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cash => "CASH",
            Self::Card => "CARD",
            Self::Deposit => "DEPOSIT",
        }
    }
}

#[derive(Debug)]
pub struct InvalidAccountType(String);

impl fmt::Display for InvalidAccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid account type: {}", self.0)
    }
}

impl std::error::Error for InvalidAccountType {}

impl TryFrom<String> for AccountType {
    type Error = InvalidAccountType;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "CASH" => Ok(Self::Cash),
            "CARD" => Ok(Self::Card),
            "DEPOSIT" => Ok(Self::Deposit),
            _ => Err(InvalidAccountType(value)),
        }
    }
}

/// Счёт пользователя (таблица `accounting_accounts`).
#[derive(Debug, Clone, FromRow)]
pub struct Account {
    #[sqlx(flatten)]
    pub base: ReferenceBase,
    pub currency_id: Uuid,
    /// Код валюты (из `accounting_currencies`)
    pub currency_code: String,
    #[sqlx(try_from = "String")]
    pub account_type: AccountType,
    pub initial_balance: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_type_round_trip() {
        for t in [AccountType::Cash, AccountType::Card, AccountType::Deposit] {
            assert_eq!(AccountType::try_from(t.as_str().to_string()).unwrap(), t);
        }
        assert!(AccountType::try_from("card".to_string()).is_err());
    }
}
