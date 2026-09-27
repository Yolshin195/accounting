//! DTO, общие для всех справочников. DTO конкретного справочника встраивают их
//! через `#[serde(flatten)]`, поэтому базовые поля у всех справочников одинаковые.

use crate::application::dtos::pagination_dto::Pagination;
use crate::domain::reference::{FieldError, ReferenceBase, ReferenceError};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

const CODE_MAX_LENGTH: usize = 64;
const NAME_MAX_LENGTH: usize = 255;

/// Базовые поля записи справочника в ответе.
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceBaseDto {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub is_archived: bool,
}

impl From<ReferenceBase> for ReferenceBaseDto {
    fn from(base: ReferenceBase) -> Self {
        Self {
            id: base.id.to_string(),
            code: base.code,
            name: base.name,
            description: base.description,
            sort_order: base.sort_order,
            is_archived: base.is_archived,
        }
    }
}

/// Базовые поля при создании записи.
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateReferenceBaseDto {
    /// Уникальный в рамках пользователя код; после создания не меняется
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub sort_order: i32,
}

impl CreateReferenceBaseDto {
    pub fn validate(&mut self, errors: &mut Vec<FieldError>) {
        normalize_required("code", &mut self.code, CODE_MAX_LENGTH, errors);
        normalize_required("name", &mut self.name, NAME_MAX_LENGTH, errors);
        normalize_optional("description", &mut self.description, None, errors);
    }
}

/// Базовые поля при обновлении записи (`code` не меняется).
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReferenceBaseDto {
    #[serde(default)]
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub sort_order: i32,
}

impl UpdateReferenceBaseDto {
    pub fn validate(&mut self, errors: &mut Vec<FieldError>) {
        normalize_required("name", &mut self.name, NAME_MAX_LENGTH, errors);
        normalize_optional("description", &mut self.description, None, errors);
    }

    /// Переносит поля в сущность.
    pub fn apply_to(self, base: &mut ReferenceBase) {
        base.name = self.name;
        base.description = self.description;
        base.sort_order = self.sort_order;
    }
}

/// Обрезает пробелы; пустая строка — ошибка «обязательное поле».
pub fn normalize_required(field: &str, value: &mut String, max_length: usize, errors: &mut Vec<FieldError>) {
    *value = value.trim().to_string();
    if value.is_empty() {
        errors.push(FieldError::new(field, "Field is required"));
    } else if value.chars().count() > max_length {
        errors.push(FieldError::new(field, format!("Must be at most {max_length} characters")));
    }
}

/// Обрезает пробелы; пустая строка превращается в `None`.
pub fn normalize_optional(field: &str, value: &mut Option<String>, max_length: Option<usize>, errors: &mut Vec<FieldError>) {
    *value = value.take().map(|v| v.trim().to_string()).filter(|v| !v.is_empty());
    if let (Some(v), Some(max)) = (value.as_ref(), max_length) {
        if v.chars().count() > max {
            errors.push(FieldError::new(field, format!("Must be at most {max} characters")));
        }
    }
}

/// `Ok`, если ошибок валидации нет.
pub fn ensure_valid(errors: Vec<FieldError>) -> Result<(), ReferenceError> {
    if errors.is_empty() {
        Ok(())
    } else {
        Err(ReferenceError::Validation(errors))
    }
}

/// Параметры списка, одинаковые для всех справочников.
#[derive(Deserialize, Debug, Clone, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct ReferenceListQuery {
    /// Номер страницы (с 0)
    #[serde(default)]
    pub page: i64,
    /// Размер страницы
    #[serde(default = "default_size")]
    pub size: i64,
    /// Поиск по вхождению в `code` или `name`
    pub q: Option<String>,
    /// Показывать архивные записи
    #[serde(default)]
    pub include_archived: bool,
}

fn default_size() -> i64 {
    Pagination::default().size
}

impl ReferenceListQuery {
    pub fn pagination(&self) -> Pagination {
        Pagination {
            page: self.page.max(0),
            size: self.size.clamp(1, 1000),
        }
    }

    pub fn search(&self) -> Option<String> {
        self.q.as_ref().map(|q| q.trim().to_string()).filter(|q| !q.is_empty())
    }
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Машиночитаемый код ошибки: `validation`, `not_found`, `conflict`, `internal`
    pub error: &'static str,
    pub message: String,
    /// Ошибки по полям (для `validation`)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<FieldError>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_base_validate_trims_and_collects_errors() {
        let mut dto = CreateReferenceBaseDto {
            code: "  ".into(),
            name: " Карта ".into(),
            description: Some("   ".into()),
            sort_order: 0,
        };
        let mut errors = Vec::new();
        dto.validate(&mut errors);

        assert_eq!(dto.name, "Карта");
        assert_eq!(dto.description, None);
        assert_eq!(errors, vec![FieldError::new("code", "Field is required")]);
    }

    #[test]
    fn test_create_base_validate_checks_length() {
        let mut dto = CreateReferenceBaseDto {
            code: "X".repeat(CODE_MAX_LENGTH + 1),
            name: "Name".into(),
            description: None,
            sort_order: 0,
        };
        let mut errors = Vec::new();
        dto.validate(&mut errors);

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "code");
    }

    #[test]
    fn test_list_query_normalizes_paging_and_search() {
        let query = ReferenceListQuery { page: -1, size: 0, q: Some("  ".into()), include_archived: false };

        assert_eq!(query.pagination().page, 0);
        assert_eq!(query.pagination().size, 1);
        assert_eq!(query.search(), None);
    }
}
