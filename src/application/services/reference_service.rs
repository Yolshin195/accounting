//! Общие операции сервисов справочников: список, получение по коду, архив, восстановление.
//! Сервис конкретного справочника вызывает их одной строкой, а сам реализует только
//! `create` и `update`.

use crate::application::dtos::pagination_dto::PagedResponse;
use crate::application::dtos::reference_dto::ReferenceListQuery;
use crate::application::traits::reference_repo::{ReferenceFilter, ReferenceRepository};
use crate::domain::reference::ReferenceError;
use uuid::Uuid;

pub async fn list_page<T, D, R>(
    repo: &R,
    user_id: Uuid,
    query: ReferenceListQuery,
    to_dto: impl Fn(T) -> D,
) -> Result<PagedResponse<D>, ReferenceError>
where
    R: ReferenceRepository<T> + ?Sized,
{
    let pagination = query.pagination();
    let filter = ReferenceFilter {
        q: query.search(),
        include_archived: query.include_archived,
    };
    let items = repo.find_all(user_id, &filter, &pagination).await?;
    let total = repo.count(user_id, &filter).await?;
    Ok(PagedResponse::new(items.into_iter().map(to_dto).collect(), &pagination, total))
}

pub async fn get_by_code<T, R>(repo: &R, user_id: Uuid, code: &str) -> Result<T, ReferenceError>
where
    R: ReferenceRepository<T> + ?Sized,
{
    repo.find_by_code(user_id, code)
        .await?
        .ok_or_else(|| ReferenceError::not_found(code))
}

/// Мягкое удаление: запись скрывается из списков и её нельзя выбрать в новых ссылках,
/// но существующие ссылки на неё остаются валидными.
pub async fn archive<T, R>(repo: &R, user_id: Uuid, code: &str) -> Result<(), ReferenceError>
where
    R: ReferenceRepository<T> + ?Sized,
{
    if repo.set_archived(user_id, code, true).await? {
        Ok(())
    } else {
        Err(ReferenceError::not_found(code))
    }
}

pub async fn restore<T, R>(repo: &R, user_id: Uuid, code: &str) -> Result<T, ReferenceError>
where
    R: ReferenceRepository<T> + ?Sized,
{
    if !repo.set_archived(user_id, code, false).await? {
        return Err(ReferenceError::not_found(code));
    }
    get_by_code(repo, user_id, code).await
}

/// Код связанной записи → её `id`; архивные и чужие записи не находятся.
pub async fn resolve_code<T, R>(repo: &R, user_id: Uuid, field: &str, code: &str) -> Result<Uuid, ReferenceError>
where
    R: ReferenceRepository<T> + ?Sized,
{
    repo.find_active_id_by_code(user_id, code)
        .await?
        .ok_or_else(|| ReferenceError::invalid(field, format!("'{code}' not found")))
}
