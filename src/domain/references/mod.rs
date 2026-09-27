//! Конкретные справочники приложения.
//!
//! Чтобы добавить новый справочник:
//! 1. миграция с таблицей: `id, user_id, code, name, description, sort_order, is_archived`,
//!    `UNIQUE (user_id, code)` + собственные колонки;
//! 2. модуль с `descriptor()` рядом с этим файлом;
//! 3. строка `.register(...)` в `registry()` ниже.
//!
//! API (`/references/{kind}`) и клиент подхватят его автоматически.

pub mod account;
pub mod currency;

use crate::domain::reference::ReferenceRegistry;

pub fn registry() -> ReferenceRegistry {
    ReferenceRegistry::new()
        .register(currency::descriptor())
        .register(account::descriptor())
}
