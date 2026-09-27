use crate::domain::reference::ReferenceBase;
use sqlx::FromRow;

/// Валюта пользователя (таблица `accounting_currencies`).
#[derive(Debug, Clone, FromRow)]
pub struct Currency {
    #[sqlx(flatten)]
    pub base: ReferenceBase,
    pub symbol: Option<String>,
    pub decimals: i32,
    pub is_base: bool,
}
