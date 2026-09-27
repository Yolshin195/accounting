//! Общие SQL-запросы справочников: список, количество, поиск по коду, архивирование.
//!
//! Репозиторий справочника передаёт сюда свою таблицу (`TABLE`) и явный `SELECT … FROM <table> t`
//! (с JOIN, если нужны поля связанных таблиц). Имена таблиц — константы из кода,
//! пользовательские данные передаются только через bind-параметры.

use crate::application::dtos::pagination_dto::Pagination;
use crate::application::traits::reference_repo::ReferenceFilter;
use crate::domain::reference::ReferenceError;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder, Row};
use uuid::Uuid;

fn push_filter<'a>(qb: &mut QueryBuilder<'a, Postgres>, user_id: Uuid, filter: &ReferenceFilter) {
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

pub async fn find_all<T>(
    pool: &PgPool,
    select: &str,
    user_id: Uuid,
    filter: &ReferenceFilter,
    pagination: &Pagination,
) -> Result<Vec<T>, ReferenceError>
where
    T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    let mut qb = QueryBuilder::new(select);
    push_filter(&mut qb, user_id, filter);
    qb.push(" ORDER BY t.sort_order, t.code LIMIT ")
        .push_bind(pagination.size)
        .push(" OFFSET ")
        .push_bind(pagination.offset());
    Ok(qb.build_query_as::<T>().fetch_all(pool).await?)
}

pub async fn count(pool: &PgPool, table: &str, user_id: Uuid, filter: &ReferenceFilter) -> Result<i64, ReferenceError> {
    let mut qb = QueryBuilder::new(format!("SELECT COUNT(*) FROM {table} t"));
    push_filter(&mut qb, user_id, filter);
    let row = qb.build().fetch_one(pool).await?;
    Ok(row.try_get(0)?)
}

pub async fn find_by_code<T>(pool: &PgPool, select: &str, user_id: Uuid, code: &str) -> Result<Option<T>, ReferenceError>
where
    T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    let mut qb = QueryBuilder::new(select);
    qb.push(" WHERE t.user_id = ")
        .push_bind(user_id)
        .push(" AND t.code = ")
        .push_bind(code.to_string());
    Ok(qb.build_query_as::<T>().fetch_optional(pool).await?)
}

pub async fn find_active_id_by_code(pool: &PgPool, table: &str, user_id: Uuid, code: &str) -> Result<Option<Uuid>, ReferenceError> {
    let mut qb = QueryBuilder::new(format!("SELECT id FROM {table} WHERE user_id = "));
    qb.push_bind(user_id)
        .push(" AND code = ")
        .push_bind(code.to_string())
        .push(" AND NOT is_archived");
    let row = qb.build().fetch_optional(pool).await?;
    Ok(row.map(|row| row.try_get("id")).transpose()?)
}

pub async fn set_archived(pool: &PgPool, table: &str, user_id: Uuid, code: &str, archived: bool) -> Result<bool, ReferenceError> {
    let mut qb = QueryBuilder::new(format!("UPDATE {table} SET is_archived = "));
    qb.push_bind(archived)
        .push(" WHERE user_id = ")
        .push_bind(user_id)
        .push(" AND code = ")
        .push_bind(code.to_string());
    let result = qb.build().execute(pool).await?;
    Ok(result.rows_affected() > 0)
}
