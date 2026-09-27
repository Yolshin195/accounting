use crate::application::dtos::pagination_dto::Pagination;
use crate::domain::reference::{
    FieldDescriptor, FieldValue, ReferenceDescriptor, ReferenceError, ReferenceRecord,
};
use async_trait::async_trait;
use uuid::Uuid;

/// Фильтр списка записей справочника.
#[derive(Debug, Clone, Default)]
pub struct ReferenceFilter {
    /// Поиск по вхождению в `code` или `name`
    pub q: Option<String>,
    pub include_archived: bool,
}

/// Хранилище справочников. Одна реализация обслуживает все справочники:
/// что и где хранить, определяет переданный дескриптор.
///
/// Значения `FieldValue::ReferenceCode` сюда не попадают — сервис заранее
/// разрешает их в `FieldValue::Uuid`. При чтении ссылочные поля возвращаются как `code`.
#[async_trait]
pub trait ReferenceRepository: Send + Sync {
    async fn find_all(
        &self,
        descriptor: &ReferenceDescriptor,
        user_id: Uuid,
        filter: &ReferenceFilter,
        pagination: &Pagination,
    ) -> Result<Vec<ReferenceRecord>, ReferenceError>;

    async fn count(
        &self,
        descriptor: &ReferenceDescriptor,
        user_id: Uuid,
        filter: &ReferenceFilter,
    ) -> Result<i64, ReferenceError>;

    async fn find_by_code(
        &self,
        descriptor: &ReferenceDescriptor,
        user_id: Uuid,
        code: &str,
    ) -> Result<Option<ReferenceRecord>, ReferenceError>;

    /// `id` неархивной записи по коду — для разрешения ссылок
    async fn find_active_id_by_code(
        &self,
        descriptor: &ReferenceDescriptor,
        user_id: Uuid,
        code: &str,
    ) -> Result<Option<Uuid>, ReferenceError>;

    async fn insert(
        &self,
        descriptor: &ReferenceDescriptor,
        user_id: Uuid,
        id: Uuid,
        values: &[(&FieldDescriptor, FieldValue)],
    ) -> Result<(), ReferenceError>;

    /// Возвращает `false`, если запись не найдена
    async fn update(
        &self,
        descriptor: &ReferenceDescriptor,
        user_id: Uuid,
        code: &str,
        values: &[(&FieldDescriptor, FieldValue)],
    ) -> Result<bool, ReferenceError>;

    /// Возвращает `false`, если запись не найдена
    async fn set_archived(
        &self,
        descriptor: &ReferenceDescriptor,
        user_id: Uuid,
        code: &str,
        archived: bool,
    ) -> Result<bool, ReferenceError>;
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::domain::reference::FieldType;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[derive(Clone)]
    struct Row {
        user_id: Uuid,
        id: Uuid,
        archived: bool,
        /// Колонка → значение (для ссылок хранится Uuid)
        columns: HashMap<&'static str, FieldValue>,
    }

    /// In-memory реализация, повторяющая поведение Postgres-репозитория
    /// (уникальность `code`, архивирование, разрешение ссылок в `code` при чтении).
    #[derive(Default)]
    pub struct MockInMemoryReferenceRepository {
        tables: Mutex<HashMap<&'static str, Vec<Row>>>,
    }

    impl MockInMemoryReferenceRepository {
        pub fn new() -> Self {
            Self::default()
        }

        fn to_record(&self, tables: &HashMap<&'static str, Vec<Row>>, d: &ReferenceDescriptor, row: &Row) -> ReferenceRecord {
            let mut record = ReferenceRecord::new();
            record.insert("id".into(), Value::String(row.id.to_string()));
            for field in &d.fields {
                let value = match row.columns.get(field.column) {
                    Some(FieldValue::Uuid(id)) if field.field_type == FieldType::Reference => tables
                        .get(field.reference_table.unwrap())
                        .and_then(|rows| rows.iter().find(|r| r.id == *id))
                        .and_then(|r| match r.columns.get("code") {
                            Some(FieldValue::Text(code)) => Some(Value::String(code.clone())),
                            _ => None,
                        })
                        .unwrap_or(Value::Null),
                    Some(FieldValue::Text(s)) => Value::String(s.clone()),
                    Some(FieldValue::Integer(i)) => Value::from(*i),
                    Some(FieldValue::Decimal(d)) => serde_json::to_value(d).unwrap(),
                    Some(FieldValue::Boolean(b)) => Value::Bool(*b),
                    Some(FieldValue::Date(d)) => Value::String(d.to_string()),
                    _ => Value::Null,
                };
                record.insert(field.name.into(), value);
            }
            record.insert("isArchived".into(), Value::Bool(row.archived));
            record
        }

        fn code_of(row: &Row) -> Option<&str> {
            match row.columns.get("code") {
                Some(FieldValue::Text(code)) => Some(code),
                _ => None,
            }
        }

        fn matches(row: &Row, user_id: Uuid, filter: &ReferenceFilter) -> bool {
            row.user_id == user_id
                && (filter.include_archived || !row.archived)
                && filter.q.as_ref().is_none_or(|q| {
                    let q = q.to_lowercase();
                    ["code", "name"].iter().any(|c| {
                        matches!(row.columns.get(c), Some(FieldValue::Text(s)) if s.to_lowercase().contains(&q))
                    })
                })
        }
    }

    #[async_trait]
    impl ReferenceRepository for MockInMemoryReferenceRepository {
        async fn find_all(&self, d: &ReferenceDescriptor, user_id: Uuid, filter: &ReferenceFilter, pagination: &Pagination) -> Result<Vec<ReferenceRecord>, ReferenceError> {
            let tables = self.tables.lock().unwrap();
            let mut rows: Vec<&Row> = tables
                .get(d.table)
                .map(|rows| rows.iter().filter(|r| Self::matches(r, user_id, filter)).collect())
                .unwrap_or_default();
            rows.sort_by_key(|r| Self::code_of(r).map(str::to_string));
            Ok(rows
                .into_iter()
                .skip(pagination.offset() as usize)
                .take(pagination.size as usize)
                .map(|r| self.to_record(&tables, d, r))
                .collect())
        }

        async fn count(&self, d: &ReferenceDescriptor, user_id: Uuid, filter: &ReferenceFilter) -> Result<i64, ReferenceError> {
            let tables = self.tables.lock().unwrap();
            Ok(tables
                .get(d.table)
                .map(|rows| rows.iter().filter(|r| Self::matches(r, user_id, filter)).count())
                .unwrap_or(0) as i64)
        }

        async fn find_by_code(&self, d: &ReferenceDescriptor, user_id: Uuid, code: &str) -> Result<Option<ReferenceRecord>, ReferenceError> {
            let tables = self.tables.lock().unwrap();
            Ok(tables
                .get(d.table)
                .and_then(|rows| rows.iter().find(|r| r.user_id == user_id && Self::code_of(r) == Some(code)))
                .map(|r| self.to_record(&tables, d, r)))
        }

        async fn find_active_id_by_code(&self, d: &ReferenceDescriptor, user_id: Uuid, code: &str) -> Result<Option<Uuid>, ReferenceError> {
            let tables = self.tables.lock().unwrap();
            Ok(tables
                .get(d.table)
                .and_then(|rows| rows.iter().find(|r| r.user_id == user_id && !r.archived && Self::code_of(r) == Some(code)))
                .map(|r| r.id))
        }

        async fn insert(&self, d: &ReferenceDescriptor, user_id: Uuid, id: Uuid, values: &[(&FieldDescriptor, FieldValue)]) -> Result<(), ReferenceError> {
            let mut tables = self.tables.lock().unwrap();
            let rows = tables.entry(d.table).or_default();
            let columns: HashMap<_, _> = values.iter().map(|(f, v)| (f.column, v.clone())).collect();
            let row = Row { user_id, id, archived: false, columns };
            if rows.iter().any(|r| r.user_id == user_id && Self::code_of(r) == Self::code_of(&row)) {
                return Err(ReferenceError::Conflict("Record with this code already exists".into()));
            }
            rows.push(row);
            Ok(())
        }

        async fn update(&self, d: &ReferenceDescriptor, user_id: Uuid, code: &str, values: &[(&FieldDescriptor, FieldValue)]) -> Result<bool, ReferenceError> {
            let mut tables = self.tables.lock().unwrap();
            let Some(row) = tables
                .get_mut(d.table)
                .and_then(|rows| rows.iter_mut().find(|r| r.user_id == user_id && Self::code_of(r) == Some(code)))
            else {
                return Ok(false);
            };
            for (f, v) in values {
                row.columns.insert(f.column, v.clone());
            }
            Ok(true)
        }

        async fn set_archived(&self, d: &ReferenceDescriptor, user_id: Uuid, code: &str, archived: bool) -> Result<bool, ReferenceError> {
            let mut tables = self.tables.lock().unwrap();
            let Some(row) = tables
                .get_mut(d.table)
                .and_then(|rows| rows.iter_mut().find(|r| r.user_id == user_id && Self::code_of(r) == Some(code)))
            else {
                return Ok(false);
            };
            row.archived = archived;
            Ok(true)
        }
    }
}
