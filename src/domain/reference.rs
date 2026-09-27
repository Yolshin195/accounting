//! Общая часть всех справочников (валюты, счета, ...).
//!
//! Каждый справочник — самостоятельная сущность со своей таблицей и своими ручками,
//! но у всех есть одинаковые базовые поля. В таблице они объявляются явно:
//!
//! ```sql
//! id          UUID PRIMARY KEY,
//! user_id     UUID    NOT NULL REFERENCES accounting_users (id),
//! code        TEXT    NOT NULL,
//! name        TEXT    NOT NULL,
//! description TEXT    NULL,
//! sort_order  INTEGER NOT NULL DEFAULT 0,
//! is_archived BOOLEAN NOT NULL DEFAULT FALSE,
//! ...
//! UNIQUE (user_id, code)
//! ```
//!
//! а в Rust встраиваются в сущность через `#[sqlx(flatten)] pub base: ReferenceBase`.

use serde::Serialize;
use sqlx::FromRow;
use std::fmt;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ReferenceBase {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub is_archived: bool,
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

/// Ошибки операций со справочниками; в HTTP превращаются в 400/404/409/500.
#[derive(Debug)]
pub enum ReferenceError {
    NotFound(String),
    Validation(Vec<FieldError>),
    Conflict(String),
    Internal(anyhow::Error),
}

impl ReferenceError {
    pub fn not_found(code: &str) -> Self {
        Self::NotFound(format!("Record '{code}' not found"))
    }

    pub fn invalid(field: &str, message: impl Into<String>) -> Self {
        Self::Validation(vec![FieldError::new(field, message)])
    }
}

impl fmt::Display for ReferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(message) | Self::Conflict(message) => write!(f, "{message}"),
            Self::Validation(errors) => {
                let details: Vec<String> = errors
                    .iter()
                    .map(|e| format!("{}: {}", e.field, e.message))
                    .collect();
                write!(f, "Validation failed: {}", details.join("; "))
            }
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

impl From<sqlx::Error> for ReferenceError {
    /// Нарушения ограничений БД — это конфликт данных, а не падение сервера.
    fn from(err: sqlx::Error) -> Self {
        if let sqlx::Error::Database(db_err) = &err {
            match db_err.code().as_deref() {
                // unique_violation
                Some("23505") => {
                    let message = match db_err.constraint() {
                        Some(c) if c.ends_with("_user_id_code_key") => {
                            "Record with this code already exists".to_string()
                        }
                        Some(c) => format!("Unique constraint violated: {c}"),
                        None => "Unique constraint violated".to_string(),
                    };
                    return Self::Conflict(message);
                }
                // foreign_key_violation
                Some("23503") => {
                    return Self::Conflict("Record is referenced by other records".to_string());
                }
                _ => {}
            }
        }
        Self::Internal(err.into())
    }
}
