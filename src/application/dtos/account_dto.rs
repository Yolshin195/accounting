use crate::application::dtos::reference_dto::{
    normalize_required, CreateReferenceBaseDto, ReferenceBaseDto, UpdateReferenceBaseDto,
};
use crate::domain::account::{Account, AccountType};
use crate::domain::reference::FieldError;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Поля счёта, общие для создания и обновления.
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccountFields {
    /// Код валюты из справочника `/currencies`
    #[serde(default)]
    pub currency: String,
    pub account_type: AccountType,
    #[serde(default)]
    pub initial_balance: Decimal,
}

impl AccountFields {
    pub fn validate(&mut self, errors: &mut Vec<FieldError>) {
        normalize_required("currency", &mut self.currency, 64, errors);
    }
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccountDto {
    #[serde(flatten)]
    pub base: ReferenceBaseDto,
    /// Код валюты
    pub currency: String,
    pub account_type: AccountType,
    pub initial_balance: Decimal,
}

impl From<Account> for AccountDto {
    fn from(account: Account) -> Self {
        Self {
            base: account.base.into(),
            currency: account.currency_code,
            account_type: account.account_type,
            initial_balance: account.initial_balance,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateAccountDto {
    #[serde(flatten)]
    pub base: CreateReferenceBaseDto,
    #[serde(flatten)]
    pub fields: AccountFields,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateAccountDto {
    #[serde(flatten)]
    pub base: UpdateReferenceBaseDto,
    #[serde(flatten)]
    pub fields: AccountFields,
}
