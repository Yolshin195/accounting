use crate::application::dtos::pagination_dto::Pagination;
use crate::domain::reference::FieldError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};

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
}

/// Запись справочника: базовые поля, общие для всех справочников,
/// плюс поля конкретного справочника из `GET /references` (`fields`).
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceItem {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub sort_order: i64,
    pub is_archived: bool,
    /// Поля конкретного справочника
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Машиночитаемый код ошибки: `unknown_reference`, `not_found`, `validation`, `conflict`, `internal`
    pub error: &'static str,
    pub message: String,
    /// Ошибки по полям (для `validation`)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<FieldError>,
}
