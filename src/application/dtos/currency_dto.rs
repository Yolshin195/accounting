use crate::application::dtos::reference_dto::{
    normalize_optional, CreateReferenceBaseDto, ReferenceBaseDto, UpdateReferenceBaseDto,
};
use crate::domain::currency::Currency;
use crate::domain::reference::FieldError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Поля валюты, общие для создания и обновления.
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyFields {
    /// Символ для отображения, например `₽`
    pub symbol: Option<String>,
    /// Количество знаков после запятой
    #[serde(default = "default_decimals")]
    pub decimals: i32,
    /// Основная валюта пользователя (может быть только одна)
    #[serde(default)]
    pub is_base: bool,
}

fn default_decimals() -> i32 {
    2
}

impl CurrencyFields {
    pub fn validate(&mut self, errors: &mut Vec<FieldError>) {
        normalize_optional("symbol", &mut self.symbol, Some(8), errors);
        if !(0..=8).contains(&self.decimals) {
            errors.push(FieldError::new("decimals", "Must be between 0 and 8"));
        }
    }

    pub fn apply_to(self, currency: &mut Currency) {
        currency.symbol = self.symbol;
        currency.decimals = self.decimals;
        currency.is_base = self.is_base;
    }
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyDto {
    #[serde(flatten)]
    pub base: ReferenceBaseDto,
    pub symbol: Option<String>,
    pub decimals: i32,
    pub is_base: bool,
}

impl From<Currency> for CurrencyDto {
    fn from(currency: Currency) -> Self {
        Self {
            base: currency.base.into(),
            symbol: currency.symbol,
            decimals: currency.decimals,
            is_base: currency.is_base,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateCurrencyDto {
    #[serde(flatten)]
    pub base: CreateReferenceBaseDto,
    #[serde(flatten)]
    pub fields: CurrencyFields,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateCurrencyDto {
    #[serde(flatten)]
    pub base: UpdateReferenceBaseDto,
    #[serde(flatten)]
    pub fields: CurrencyFields,
}
