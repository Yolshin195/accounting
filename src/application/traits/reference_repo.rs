use crate::application::dtos::pagination_dto::Pagination;
use crate::domain::reference::ReferenceError;
use async_trait::async_trait;
use uuid::Uuid;

/// Фильтр списка записей справочника.
#[derive(Debug, Clone, Default)]
pub struct ReferenceFilter {
    /// Поиск по вхождению в `code` или `name`
    pub q: Option<String>,
    pub include_archived: bool,
}

/// Операции, одинаковые для всех справочников. Репозиторий конкретного справочника
/// расширяет этот трейт своими `insert`/`update`:
///
/// ```ignore
/// pub trait CurrencyRepository: ReferenceRepository<Currency> { ... }
/// ```
///
/// Postgres-реализация этих методов — однострочники через `infrastructure::db::reference_queries`.
#[async_trait]
pub trait ReferenceRepository<T>: Send + Sync {
    async fn find_all(&self, user_id: Uuid, filter: &ReferenceFilter, pagination: &Pagination) -> Result<Vec<T>, ReferenceError>;

    async fn count(&self, user_id: Uuid, filter: &ReferenceFilter) -> Result<i64, ReferenceError>;

    async fn find_by_code(&self, user_id: Uuid, code: &str) -> Result<Option<T>, ReferenceError>;

    /// `id` неархивной записи по коду — для ссылок из других сущностей
    async fn find_active_id_by_code(&self, user_id: Uuid, code: &str) -> Result<Option<Uuid>, ReferenceError>;

    /// Возвращает `false`, если запись не найдена
    async fn set_archived(&self, user_id: Uuid, code: &str, archived: bool) -> Result<bool, ReferenceError>;
}
