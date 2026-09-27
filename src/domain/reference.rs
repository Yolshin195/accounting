//! Общая модель справочников.
//!
//! Справочник целиком описывается `ReferenceDescriptor`: какие у него поля, как они
//! называются в JSON и в БД, какого они типа. По дескриптору:
//! - клиент строит меню, таблицу и форму (`GET /references`);
//! - сервис валидирует входящий JSON;
//! - generic-репозиторий строит SQL.
//!
//! Поэтому новый справочник = миграция + дескриптор (см. `domain::references`).

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Serialize;
use serde_json::{Map, Value};
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

/// Запись справочника в виде плоского JSON-объекта:
/// базовые поля (`id`, `code`, `name`, `description`, `sortOrder`, `isArchived`) + поля справочника.
pub type ReferenceRecord = Map<String, Value>;

/// Тип поля — определяет виджет на клиенте, валидацию и тип колонки в БД.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    /// Однострочный текст (TEXT)
    String,
    /// Многострочный текст (TEXT)
    Text,
    /// Целое число (INTEGER / BIGINT)
    Integer,
    /// Десятичное число (DECIMAL); в ответах — строка, как и суммы транзакций
    Decimal,
    /// Флаг (BOOLEAN)
    Boolean,
    /// Дата `YYYY-MM-DD` (DATE)
    #[allow(dead_code)] // пока ни один справочник не использует даты
    Date,
    /// Значение из фиксированного списка `options` (TEXT)
    Enum,
    /// Ссылка на запись другого справочника по `code` (UUID FK на id)
    Reference,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct EnumOption {
    pub value: &'static str,
    pub label: &'static str,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FieldDescriptor {
    /// Ключ поля в JSON
    pub name: &'static str,
    /// Колонка в таблице справочника
    #[serde(skip)]
    pub column: &'static str,
    /// Подпись поля на клиенте
    pub title: &'static str,
    #[serde(rename = "type")]
    pub field_type: FieldType,
    pub required: bool,
    /// Поле задаётся только при создании (например, `code`)
    pub readonly_on_update: bool,
    /// Для `reference`: kind справочника, на который ссылается поле
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'static str>,
    /// Для `reference`: таблица целевого справочника
    #[serde(skip)]
    pub reference_table: Option<&'static str>,
    /// Для `enum`: допустимые значения
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<EnumOption>,
    /// Значение по умолчанию, если поле не передано
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
}

impl FieldDescriptor {
    pub fn new(field_type: FieldType, name: &'static str, column: &'static str, title: &'static str) -> Self {
        Self {
            name,
            column,
            title,
            field_type,
            required: false,
            readonly_on_update: false,
            reference: None,
            reference_table: None,
            options: Vec::new(),
            default: None,
            max_length: None,
        }
    }

    pub fn string(name: &'static str, column: &'static str, title: &'static str) -> Self {
        Self::new(FieldType::String, name, column, title).max_length(255)
    }

    pub fn text(name: &'static str, column: &'static str, title: &'static str) -> Self {
        Self::new(FieldType::Text, name, column, title)
    }

    pub fn integer(name: &'static str, column: &'static str, title: &'static str) -> Self {
        Self::new(FieldType::Integer, name, column, title)
    }

    pub fn decimal(name: &'static str, column: &'static str, title: &'static str) -> Self {
        Self::new(FieldType::Decimal, name, column, title)
    }

    pub fn boolean(name: &'static str, column: &'static str, title: &'static str) -> Self {
        Self::new(FieldType::Boolean, name, column, title).default_value(false)
    }

    #[allow(dead_code)]
    pub fn date(name: &'static str, column: &'static str, title: &'static str) -> Self {
        Self::new(FieldType::Date, name, column, title)
    }

    pub fn enumeration(
        name: &'static str,
        column: &'static str,
        title: &'static str,
        options: &[(&'static str, &'static str)],
    ) -> Self {
        let mut field = Self::new(FieldType::Enum, name, column, title);
        field.options = options
            .iter()
            .map(|&(value, label)| EnumOption { value, label })
            .collect();
        field
    }

    /// Ссылка на другой справочник. В JSON передаётся `code` целевой записи,
    /// в БД хранится её `id` в колонке `column`.
    pub fn reference(
        name: &'static str,
        column: &'static str,
        title: &'static str,
        target: &ReferenceDescriptor,
    ) -> Self {
        let mut field = Self::new(FieldType::Reference, name, column, title);
        field.reference = Some(target.kind);
        field.reference_table = Some(target.table);
        field
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn readonly_on_update(mut self) -> Self {
        self.readonly_on_update = true;
        self
    }

    pub fn default_value(mut self, value: impl Into<Value>) -> Self {
        self.default = Some(value.into());
        self
    }

    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceDescriptor {
    /// Идентификатор справочника в URL: `/references/{kind}`
    pub kind: &'static str,
    pub title: &'static str,
    pub icon: &'static str,
    #[serde(skip)]
    pub table: &'static str,
    /// Все редактируемые поля, включая базовые
    pub fields: Vec<FieldDescriptor>,
    /// Колонки для таблицы на клиенте
    pub list_columns: Vec<&'static str>,
}

impl ReferenceDescriptor {
    /// Создаёт дескриптор с базовыми полями, общими для всех справочников:
    /// `code`, `name`, `description`, `sortOrder`.
    /// Таблица обязана содержать колонки `id, user_id, code, name, description,
    /// sort_order, is_archived` и `UNIQUE (user_id, code)`.
    pub fn new(kind: &'static str, table: &'static str, title: &'static str, icon: &'static str) -> Self {
        Self {
            kind,
            title,
            icon,
            table,
            fields: vec![
                FieldDescriptor::string("code", "code", "Код")
                    .max_length(64)
                    .required()
                    .readonly_on_update(),
                FieldDescriptor::string("name", "name", "Название").required(),
                FieldDescriptor::text("description", "description", "Описание"),
                FieldDescriptor::integer("sortOrder", "sort_order", "Порядок").default_value(0),
            ],
            list_columns: vec!["code", "name"],
        }
    }

    pub fn field(mut self, field: FieldDescriptor) -> Self {
        self.fields.push(field);
        self
    }

    pub fn list_columns(mut self, columns: &[&'static str]) -> Self {
        self.list_columns = columns.to_vec();
        self
    }
}

/// Реестр всех справочников приложения.
#[derive(Debug, Default)]
pub struct ReferenceRegistry {
    items: Vec<Arc<ReferenceDescriptor>>,
}

impl ReferenceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Регистрирует справочник. Справочник, на который ссылаются поля,
    /// должен быть зарегистрирован раньше; ошибки в дескрипторах — ошибка программиста,
    /// поэтому они приводят к панике при старте.
    pub fn register(mut self, descriptor: ReferenceDescriptor) -> Self {
        assert!(
            self.get(descriptor.kind).is_none(),
            "Reference '{}' is registered twice",
            descriptor.kind
        );
        for field in &descriptor.fields {
            if let Some(target) = field.reference {
                assert!(
                    self.get(target).is_some(),
                    "Reference '{}.{}' points to unregistered reference '{}'",
                    descriptor.kind,
                    field.name,
                    target
                );
            }
            if field.field_type == FieldType::Enum {
                assert!(
                    !field.options.is_empty(),
                    "Enum field '{}.{}' has no options",
                    descriptor.kind,
                    field.name
                );
            }
        }
        self.items.push(Arc::new(descriptor));
        self
    }

    pub fn get(&self, kind: &str) -> Option<Arc<ReferenceDescriptor>> {
        self.items.iter().find(|d| d.kind == kind).cloned()
    }

    pub fn all(&self) -> &[Arc<ReferenceDescriptor>] {
        &self.items
    }
}

/// Значение поля после валидации, готовое к записи в БД.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    Null,
    Text(String),
    Integer(i64),
    Decimal(Decimal),
    Boolean(bool),
    Date(NaiveDate),
    /// Ссылка до разрешения: `code` записи целевого справочника
    ReferenceCode(String),
    /// Ссылка после разрешения: `id` записи целевого справочника
    Uuid(Uuid),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteMode {
    Create,
    Update,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

impl FieldError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug)]
pub enum ReferenceError {
    UnknownKind(String),
    NotFound { kind: String, code: String },
    Validation(Vec<FieldError>),
    Conflict(String),
    Internal(anyhow::Error),
}

impl fmt::Display for ReferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownKind(kind) => write!(f, "Reference '{kind}' does not exist"),
            Self::NotFound { kind, code } => write!(f, "Record '{code}' not found in reference '{kind}'"),
            Self::Validation(errors) => {
                let details: Vec<String> = errors.iter().map(|e| format!("{}: {}", e.field, e.message)).collect();
                write!(f, "Validation failed: {}", details.join("; "))
            }
            Self::Conflict(message) => write!(f, "{message}"),
            Self::Internal(err) => write!(f, "Internal error: {err}"),
        }
    }
}

impl std::error::Error for ReferenceError {}

impl From<anyhow::Error> for ReferenceError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err)
    }
}

/// Проверяет входящий JSON по дескриптору и приводит значения к типам полей.
///
/// Семантика полная (как у PUT): не переданное поле получает `default` или `null`.
/// При обновлении поля с `readonly_on_update` пропускаются. Неизвестные ключи игнорируются,
/// чтобы клиент мог отправлять обратно полученную запись целиком.
pub fn parse_input<'a>(
    descriptor: &'a ReferenceDescriptor,
    input: &Value,
    mode: WriteMode,
) -> Result<Vec<(&'a FieldDescriptor, FieldValue)>, ReferenceError> {
    let Some(input) = input.as_object() else {
        return Err(ReferenceError::Validation(vec![FieldError::new(
            "$",
            "Request body must be a JSON object",
        )]));
    };

    let mut values = Vec::with_capacity(descriptor.fields.len());
    let mut errors = Vec::new();

    for field in &descriptor.fields {
        if mode == WriteMode::Update && field.readonly_on_update {
            continue;
        }
        let raw = match input.get(field.name) {
            Some(Value::Null) | None => field.default.as_ref().unwrap_or(&Value::Null),
            Some(value) => value,
        };
        match parse_value(field, raw) {
            Ok(FieldValue::Null) if field.required => {
                errors.push(FieldError::new(field.name, "Field is required"))
            }
            Ok(value) => values.push((field, value)),
            Err(message) => errors.push(FieldError::new(field.name, message)),
        }
    }

    if errors.is_empty() {
        Ok(values)
    } else {
        Err(ReferenceError::Validation(errors))
    }
}

fn parse_value(field: &FieldDescriptor, raw: &Value) -> Result<FieldValue, String> {
    if raw.is_null() {
        return Ok(FieldValue::Null);
    }
    match field.field_type {
        FieldType::String | FieldType::Text => {
            let s = raw.as_str().ok_or("Expected a string")?.trim();
            if s.is_empty() {
                return Ok(FieldValue::Null);
            }
            if let Some(max) = field.max_length {
                if s.chars().count() > max {
                    return Err(format!("Must be at most {max} characters"));
                }
            }
            Ok(FieldValue::Text(s.to_string()))
        }
        FieldType::Integer => raw
            .as_i64()
            .map(FieldValue::Integer)
            .ok_or_else(|| "Expected an integer".to_string()),
        FieldType::Decimal => {
            let s = match raw {
                Value::Number(n) => n.to_string(),
                Value::String(s) => s.trim().to_string(),
                _ => return Err("Expected a number".to_string()),
            };
            Decimal::from_str(&s)
                .or_else(|_| Decimal::from_scientific(&s))
                .map(FieldValue::Decimal)
                .map_err(|_| "Expected a number".to_string())
        }
        FieldType::Boolean => raw
            .as_bool()
            .map(FieldValue::Boolean)
            .ok_or_else(|| "Expected a boolean".to_string()),
        FieldType::Date => {
            let s = raw.as_str().ok_or("Expected a date string YYYY-MM-DD")?;
            NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d")
                .map(FieldValue::Date)
                .map_err(|_| "Expected a date string YYYY-MM-DD".to_string())
        }
        FieldType::Enum => {
            let s = raw.as_str().ok_or("Expected a string")?.trim();
            if field.options.iter().any(|o| o.value == s) {
                Ok(FieldValue::Text(s.to_string()))
            } else {
                let allowed: Vec<&str> = field.options.iter().map(|o| o.value).collect();
                Err(format!("Must be one of: {}", allowed.join(", ")))
            }
        }
        FieldType::Reference => {
            let s = raw.as_str().ok_or("Expected a code string")?.trim();
            if s.is_empty() {
                Ok(FieldValue::Null)
            } else {
                Ok(FieldValue::ReferenceCode(s.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn descriptor() -> ReferenceDescriptor {
        let target = ReferenceDescriptor::new("targets", "targets", "Targets", "box");
        ReferenceDescriptor::new("things", "things", "Things", "box")
            .field(FieldDescriptor::enumeration("kind", "kind", "Kind", &[("A", "a"), ("B", "b")]).required())
            .field(FieldDescriptor::decimal("amount", "amount", "Amount").default_value(0))
            .field(FieldDescriptor::reference("target", "target_id", "Target", &target))
    }

    fn value_of<'a>(values: &'a [(&FieldDescriptor, FieldValue)], name: &str) -> Option<&'a FieldValue> {
        values.iter().find(|(f, _)| f.name == name).map(|(_, v)| v)
    }

    #[test]
    fn test_parse_input_valid_with_defaults() {
        let d = descriptor();
        let values = parse_input(
            &d,
            &json!({"code": " X1 ", "name": "Thing", "kind": "A", "target": "T1", "unknown": 1}),
            WriteMode::Create,
        )
        .unwrap();

        assert_eq!(value_of(&values, "code"), Some(&FieldValue::Text("X1".into())));
        assert_eq!(value_of(&values, "description"), Some(&FieldValue::Null));
        assert_eq!(value_of(&values, "sortOrder"), Some(&FieldValue::Integer(0)));
        assert_eq!(value_of(&values, "amount"), Some(&FieldValue::Decimal(Decimal::ZERO)));
        assert_eq!(value_of(&values, "target"), Some(&FieldValue::ReferenceCode("T1".into())));
    }

    #[test]
    fn test_parse_input_collects_all_errors() {
        let d = descriptor();
        let err = parse_input(&d, &json!({"code": "", "kind": "C", "amount": "abc"}), WriteMode::Create)
            .unwrap_err();

        let ReferenceError::Validation(errors) = err else { panic!("expected validation error") };
        let fields: Vec<&str> = errors.iter().map(|e| e.field.as_str()).collect();
        assert_eq!(fields, vec!["code", "name", "kind", "amount"]);
    }

    #[test]
    fn test_parse_input_update_skips_readonly_fields() {
        let d = descriptor();
        let values = parse_input(&d, &json!({"name": "Thing", "kind": "B", "amount": 1.5}), WriteMode::Update).unwrap();

        assert!(value_of(&values, "code").is_none());
        assert_eq!(value_of(&values, "amount"), Some(&FieldValue::Decimal(Decimal::new(15, 1))));
    }

    #[test]
    fn test_parse_input_rejects_non_object() {
        let d = descriptor();
        assert!(matches!(
            parse_input(&d, &json!([1, 2]), WriteMode::Create),
            Err(ReferenceError::Validation(_))
        ));
    }

    #[test]
    #[should_panic(expected = "unregistered reference")]
    fn test_registry_rejects_unknown_reference_target() {
        ReferenceRegistry::new().register(descriptor());
    }
}
