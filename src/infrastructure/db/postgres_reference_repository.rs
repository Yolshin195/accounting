use crate::application::dtos::pagination_dto::Pagination;
use crate::application::traits::reference_repo::{ReferenceFilter, ReferenceRepository};
use crate::domain::reference::{
    FieldDescriptor, FieldType, FieldValue, ReferenceDescriptor, ReferenceError, ReferenceRecord,
};
use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde_json::Value;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Postgres, QueryBuilder, Row};
use uuid::Uuid;

/// Generic-репозиторий: SQL строится по дескриптору справочника.
///
/// Имена таблиц и колонок берутся только из дескрипторов в коде (`domain::references`),
/// пользовательские данные всегда передаются через bind-параметры.
#[derive(Clone)]
pub struct PostgresReferenceRepository {
    pub pool: PgPool,
}

impl PostgresReferenceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn push_select<'a>(qb: &mut QueryBuilder<'a, Postgres>, d: &ReferenceDescriptor) {
    qb.push("SELECT t.id, t.is_archived");
    for field in &d.fields {
        match (field.field_type, field.reference_table) {
            (FieldType::Reference, Some(target)) => qb.push(format!(
                ", (SELECT r.code FROM {target} r WHERE r.id = t.{col}) AS {col}",
                col = field.column
            )),
            _ => qb.push(format!(", t.{}", field.column)),
        };
    }
    qb.push(format!(" FROM {} t", d.table));
}

fn push_filter<'a>(qb: &mut QueryBuilder<'a, Postgres>, user_id: Uuid, filter: &'a ReferenceFilter) {
    qb.push(" WHERE t.user_id = ").push_bind(user_id);
    if !filter.include_archived {
        qb.push(" AND NOT t.is_archived");
    }
    if let Some(q) = &filter.q {
        let pattern = format!("%{}%", q.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_"));
        qb.push(" AND (t.code ILIKE ")
            .push_bind(pattern.clone())
            .push(" OR t.name ILIKE ")
            .push_bind(pattern)
            .push(")");
    }
}

/// Биндит значение с типом, соответствующим типу поля (важно для NULL).
fn push_value<'a>(qb: &mut QueryBuilder<'a, Postgres>, field: &FieldDescriptor, value: &FieldValue) {
    match (value, field.field_type) {
        (FieldValue::Text(s), _) => qb.push_bind(s.clone()),
        (FieldValue::Integer(i), _) => qb.push_bind(*i),
        (FieldValue::Decimal(d), _) => qb.push_bind(*d),
        (FieldValue::Boolean(b), _) => qb.push_bind(*b),
        (FieldValue::Date(d), _) => qb.push_bind(*d),
        (FieldValue::Uuid(id), _) => qb.push_bind(*id),
        (FieldValue::ReferenceCode(_), _) => unreachable!("reference codes are resolved by the service"),
        (FieldValue::Null, FieldType::String | FieldType::Text | FieldType::Enum) => qb.push_bind(None::<String>),
        (FieldValue::Null, FieldType::Integer) => qb.push_bind(None::<i64>),
        (FieldValue::Null, FieldType::Decimal) => qb.push_bind(None::<Decimal>),
        (FieldValue::Null, FieldType::Boolean) => qb.push_bind(None::<bool>),
        (FieldValue::Null, FieldType::Date) => qb.push_bind(None::<NaiveDate>),
        (FieldValue::Null, FieldType::Reference) => qb.push_bind(None::<Uuid>),
    };
}

fn row_to_record(d: &ReferenceDescriptor, row: &PgRow) -> Result<ReferenceRecord, sqlx::Error> {
    let mut record = ReferenceRecord::new();
    record.insert("id".into(), Value::String(row.try_get::<Uuid, _>("id")?.to_string()));
    for field in &d.fields {
        let col = field.column;
        let value = match field.field_type {
            FieldType::String | FieldType::Text | FieldType::Enum | FieldType::Reference => {
                row.try_get::<Option<String>, _>(col)?.map(Value::String)
            }
            FieldType::Integer => match row.try_get::<Option<i64>, _>(col) {
                Ok(v) => v.map(Value::from),
                Err(_) => row.try_get::<Option<i32>, _>(col)?.map(Value::from),
            },
            FieldType::Decimal => row
                .try_get::<Option<Decimal>, _>(col)?
                .map(|v| serde_json::to_value(v).unwrap_or(Value::Null)),
            FieldType::Boolean => row.try_get::<Option<bool>, _>(col)?.map(Value::Bool),
            FieldType::Date => row.try_get::<Option<NaiveDate>, _>(col)?.map(|v| Value::String(v.to_string())),
        };
        record.insert(field.name.into(), value.unwrap_or(Value::Null));
    }
    record.insert("isArchived".into(), Value::Bool(row.try_get("is_archived")?));
    Ok(record)
}

fn map_db_error(err: sqlx::Error) -> ReferenceError {
    if let sqlx::Error::Database(db_err) = &err {
        match db_err.code().as_deref() {
            // unique_violation
            Some("23505") => {
                let message = match db_err.constraint() {
                    Some(c) if c.ends_with("_user_id_code_key") => "Record with this code already exists".to_string(),
                    Some(c) => format!("Unique constraint violated: {c}"),
                    None => "Unique constraint violated".to_string(),
                };
                return ReferenceError::Conflict(message);
            }
            // foreign_key_violation
            Some("23503") => return ReferenceError::Conflict("Record is referenced by other records".to_string()),
            _ => {}
        }
    }
    ReferenceError::Internal(err.into())
}

#[async_trait]
impl ReferenceRepository for PostgresReferenceRepository {
    async fn find_all(
        &self,
        d: &ReferenceDescriptor,
        user_id: Uuid,
        filter: &ReferenceFilter,
        pagination: &Pagination,
    ) -> Result<Vec<ReferenceRecord>, ReferenceError> {
        let mut qb = QueryBuilder::new("");
        push_select(&mut qb, d);
        push_filter(&mut qb, user_id, filter);
        qb.push(" ORDER BY t.sort_order, t.code LIMIT ")
            .push_bind(pagination.size)
            .push(" OFFSET ")
            .push_bind(pagination.offset());

        let rows = qb.build().fetch_all(&self.pool).await.map_err(map_db_error)?;
        rows.iter()
            .map(|row| row_to_record(d, row).map_err(map_db_error))
            .collect()
    }

    async fn count(&self, d: &ReferenceDescriptor, user_id: Uuid, filter: &ReferenceFilter) -> Result<i64, ReferenceError> {
        let mut qb = QueryBuilder::new(format!("SELECT COUNT(*) FROM {} t", d.table));
        push_filter(&mut qb, user_id, filter);
        let row = qb.build().fetch_one(&self.pool).await.map_err(map_db_error)?;
        row.try_get(0).map_err(map_db_error)
    }

    async fn find_by_code(&self, d: &ReferenceDescriptor, user_id: Uuid, code: &str) -> Result<Option<ReferenceRecord>, ReferenceError> {
        let mut qb = QueryBuilder::new("");
        push_select(&mut qb, d);
        qb.push(" WHERE t.user_id = ")
            .push_bind(user_id)
            .push(" AND t.code = ")
            .push_bind(code.to_string());

        let row = qb.build().fetch_optional(&self.pool).await.map_err(map_db_error)?;
        row.map(|row| row_to_record(d, &row).map_err(map_db_error)).transpose()
    }

    async fn find_active_id_by_code(&self, d: &ReferenceDescriptor, user_id: Uuid, code: &str) -> Result<Option<Uuid>, ReferenceError> {
        let mut qb = QueryBuilder::new(format!("SELECT id FROM {} WHERE user_id = ", d.table));
        qb.push_bind(user_id)
            .push(" AND code = ")
            .push_bind(code.to_string())
            .push(" AND NOT is_archived");

        let row = qb.build().fetch_optional(&self.pool).await.map_err(map_db_error)?;
        row.map(|row| row.try_get::<Uuid, _>("id").map_err(map_db_error)).transpose()
    }

    async fn insert(
        &self,
        d: &ReferenceDescriptor,
        user_id: Uuid,
        id: Uuid,
        values: &[(&FieldDescriptor, FieldValue)],
    ) -> Result<(), ReferenceError> {
        let mut qb = QueryBuilder::new(format!("INSERT INTO {} (id, user_id", d.table));
        for (field, _) in values {
            qb.push(", ").push(field.column);
        }
        qb.push(") VALUES (").push_bind(id).push(", ").push_bind(user_id);
        for (field, value) in values {
            qb.push(", ");
            push_value(&mut qb, field, value);
        }
        qb.push(")");

        qb.build().execute(&self.pool).await.map_err(map_db_error)?;
        Ok(())
    }

    async fn update(
        &self,
        d: &ReferenceDescriptor,
        user_id: Uuid,
        code: &str,
        values: &[(&FieldDescriptor, FieldValue)],
    ) -> Result<bool, ReferenceError> {
        if values.is_empty() {
            return Ok(self.find_active_id_by_code(d, user_id, code).await?.is_some());
        }
        let mut qb = QueryBuilder::new(format!("UPDATE {} SET ", d.table));
        for (i, (field, value)) in values.iter().enumerate() {
            if i > 0 {
                qb.push(", ");
            }
            qb.push(field.column).push(" = ");
            push_value(&mut qb, field, value);
        }
        qb.push(" WHERE user_id = ")
            .push_bind(user_id)
            .push(" AND code = ")
            .push_bind(code.to_string());

        let result = qb.build().execute(&self.pool).await.map_err(map_db_error)?;
        Ok(result.rows_affected() > 0)
    }

    async fn set_archived(&self, d: &ReferenceDescriptor, user_id: Uuid, code: &str, archived: bool) -> Result<bool, ReferenceError> {
        let mut qb = QueryBuilder::new(format!("UPDATE {} SET is_archived = ", d.table));
        qb.push_bind(archived)
            .push(" WHERE user_id = ")
            .push_bind(user_id)
            .push(" AND code = ")
            .push_bind(code.to_string());

        let result = qb.build().execute(&self.pool).await.map_err(map_db_error)?;
        Ok(result.rows_affected() > 0)
    }
}
