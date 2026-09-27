use crate::application::dtos::pagination_dto::PagedResponse;
use crate::application::dtos::reference_dto::ReferenceListQuery;
use crate::application::traits::reference_repo::{ReferenceFilter, ReferenceRepository};
use crate::domain::reference::{
    parse_input, FieldDescriptor, FieldError, FieldValue, ReferenceDescriptor, ReferenceError,
    ReferenceRecord, ReferenceRegistry, WriteMode,
};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

/// Единый сервис для всех справочников из реестра.
#[derive(Clone)]
pub struct ReferenceService {
    registry: Arc<ReferenceRegistry>,
    repo: Arc<dyn ReferenceRepository>,
}

impl ReferenceService {
    pub fn new(registry: ReferenceRegistry, repo: Arc<dyn ReferenceRepository>) -> Self {
        Self {
            registry: Arc::new(registry),
            repo,
        }
    }

    pub fn descriptors(&self) -> Vec<ReferenceDescriptor> {
        self.registry.all().iter().map(|d| (**d).clone()).collect()
    }

    pub fn descriptor(&self, kind: &str) -> Result<Arc<ReferenceDescriptor>, ReferenceError> {
        self.registry
            .get(kind)
            .ok_or_else(|| ReferenceError::UnknownKind(kind.to_string()))
    }

    pub async fn list(
        &self,
        kind: &str,
        user_id: Uuid,
        query: ReferenceListQuery,
    ) -> Result<PagedResponse<ReferenceRecord>, ReferenceError> {
        let d = self.descriptor(kind)?;
        let pagination = query.pagination();
        let filter = ReferenceFilter {
            q: query.q.map(|q| q.trim().to_string()).filter(|q| !q.is_empty()),
            include_archived: query.include_archived,
        };
        let items = self.repo.find_all(&d, user_id, &filter, &pagination).await?;
        let total = self.repo.count(&d, user_id, &filter).await?;
        Ok(PagedResponse::new(items, &pagination, total))
    }

    pub async fn get(&self, kind: &str, user_id: Uuid, code: &str) -> Result<ReferenceRecord, ReferenceError> {
        let d = self.descriptor(kind)?;
        self.find_existing(&d, user_id, code).await
    }

    pub async fn create(&self, kind: &str, user_id: Uuid, input: Value) -> Result<ReferenceRecord, ReferenceError> {
        let d = self.descriptor(kind)?;
        let values = parse_input(&d, &input, WriteMode::Create)?;
        let values = self.resolve_references(user_id, values).await?;
        let code = values
            .iter()
            .find_map(|(f, v)| match (f.name, v) {
                ("code", FieldValue::Text(code)) => Some(code.clone()),
                _ => None,
            })
            .ok_or_else(|| ReferenceError::Validation(vec![FieldError::new("code", "Field is required")]))?;

        self.repo.insert(&d, user_id, Uuid::new_v4(), &values).await?;
        self.find_existing(&d, user_id, &code).await
    }

    /// Полное обновление записи; `code` не меняется.
    pub async fn update(&self, kind: &str, user_id: Uuid, code: &str, input: Value) -> Result<ReferenceRecord, ReferenceError> {
        let d = self.descriptor(kind)?;
        let values = parse_input(&d, &input, WriteMode::Update)?;
        let values = self.resolve_references(user_id, values).await?;
        if !self.repo.update(&d, user_id, code, &values).await? {
            return Err(not_found(&d, code));
        }
        self.find_existing(&d, user_id, code).await
    }

    /// Мягкое удаление: запись скрывается из списков и её нельзя выбрать в ссылках,
    /// но существующие ссылки на неё остаются валидными.
    pub async fn archive(&self, kind: &str, user_id: Uuid, code: &str) -> Result<(), ReferenceError> {
        let d = self.descriptor(kind)?;
        if !self.repo.set_archived(&d, user_id, code, true).await? {
            return Err(not_found(&d, code));
        }
        Ok(())
    }

    pub async fn restore(&self, kind: &str, user_id: Uuid, code: &str) -> Result<ReferenceRecord, ReferenceError> {
        let d = self.descriptor(kind)?;
        if !self.repo.set_archived(&d, user_id, code, false).await? {
            return Err(not_found(&d, code));
        }
        self.find_existing(&d, user_id, code).await
    }

    async fn find_existing(&self, d: &ReferenceDescriptor, user_id: Uuid, code: &str) -> Result<ReferenceRecord, ReferenceError> {
        self.repo
            .find_by_code(d, user_id, code)
            .await?
            .ok_or_else(|| not_found(d, code))
    }

    /// Заменяет `code` ссылок на `id` записей целевых справочников.
    async fn resolve_references<'a>(
        &self,
        user_id: Uuid,
        values: Vec<(&'a FieldDescriptor, FieldValue)>,
    ) -> Result<Vec<(&'a FieldDescriptor, FieldValue)>, ReferenceError> {
        let mut resolved = Vec::with_capacity(values.len());
        let mut errors = Vec::new();

        for (field, value) in values {
            let FieldValue::ReferenceCode(code) = &value else {
                resolved.push((field, value));
                continue;
            };
            let target = field
                .reference
                .and_then(|kind| self.registry.get(kind))
                .ok_or_else(|| anyhow::anyhow!("Reference target of '{}' is not registered", field.name))?;
            match self.repo.find_active_id_by_code(&target, user_id, code).await? {
                Some(id) => resolved.push((field, FieldValue::Uuid(id))),
                None => errors.push(FieldError::new(
                    field.name,
                    format!("'{code}' not found in reference '{}'", target.kind),
                )),
            }
        }

        if errors.is_empty() {
            Ok(resolved)
        } else {
            Err(ReferenceError::Validation(errors))
        }
    }
}

fn not_found(d: &ReferenceDescriptor, code: &str) -> ReferenceError {
    ReferenceError::NotFound {
        kind: d.kind.to_string(),
        code: code.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::traits::reference_repo::mock::MockInMemoryReferenceRepository;
    use crate::domain::references;
    use serde_json::json;

    fn service() -> ReferenceService {
        ReferenceService::new(references::registry(), Arc::new(MockInMemoryReferenceRepository::new()))
    }

    fn list_query() -> ReferenceListQuery {
        ReferenceListQuery { page: 0, size: 100, q: None, include_archived: false }
    }

    #[tokio::test]
    async fn test_create_currency_applies_defaults() {
        let service = service();
        let user_id = Uuid::new_v4();

        let created = service
            .create("currencies", user_id, json!({"code": "RUB", "name": "Рубль", "symbol": "₽"}))
            .await
            .unwrap();

        assert_eq!(created["code"], "RUB");
        assert_eq!(created["decimals"], 2);
        assert_eq!(created["isBase"], false);
        assert_eq!(created["isArchived"], false);
    }

    #[tokio::test]
    async fn test_create_duplicate_code_is_conflict() {
        let service = service();
        let user_id = Uuid::new_v4();
        let body = json!({"code": "RUB", "name": "Рубль"});

        service.create("currencies", user_id, body.clone()).await.unwrap();
        let err = service.create("currencies", user_id, body.clone()).await.unwrap_err();
        assert!(matches!(err, ReferenceError::Conflict(_)));

        // У другого пользователя тот же код допустим
        service.create("currencies", Uuid::new_v4(), body).await.unwrap();
    }

    #[tokio::test]
    async fn test_account_references_currency_by_code() {
        let service = service();
        let user_id = Uuid::new_v4();
        service.create("currencies", user_id, json!({"code": "THB", "name": "Бат"})).await.unwrap();

        let account = service
            .create(
                "accounts",
                user_id,
                json!({"code": "CASH", "name": "Наличные", "currency": "THB", "accountType": "CASH"}),
            )
            .await
            .unwrap();

        assert_eq!(account["currency"], "THB");
        assert_eq!(account["initialBalance"], "0");
    }

    #[tokio::test]
    async fn test_account_with_unknown_currency_is_validation_error() {
        let service = service();
        let user_id = Uuid::new_v4();
        // Валюта другого пользователя не видна
        service.create("currencies", Uuid::new_v4(), json!({"code": "THB", "name": "Бат"})).await.unwrap();

        let err = service
            .create(
                "accounts",
                user_id,
                json!({"code": "CASH", "name": "Наличные", "currency": "THB", "accountType": "CASH"}),
            )
            .await
            .unwrap_err();

        let ReferenceError::Validation(errors) = err else { panic!("expected validation error") };
        assert_eq!(errors[0].field, "currency");
    }

    #[tokio::test]
    async fn test_update_keeps_code() {
        let service = service();
        let user_id = Uuid::new_v4();
        service.create("currencies", user_id, json!({"code": "USD", "name": "Dollar"})).await.unwrap();

        let updated = service
            .update("currencies", user_id, "USD", json!({"code": "EUR", "name": "US Dollar", "decimals": 2}))
            .await
            .unwrap();

        assert_eq!(updated["code"], "USD");
        assert_eq!(updated["name"], "US Dollar");
    }

    #[tokio::test]
    async fn test_archive_hides_record_and_forbids_new_references() {
        let service = service();
        let user_id = Uuid::new_v4();
        service.create("currencies", user_id, json!({"code": "USD", "name": "Dollar"})).await.unwrap();

        service.archive("currencies", user_id, "USD").await.unwrap();

        let page = service.list("currencies", user_id, list_query()).await.unwrap();
        assert!(page.content.is_empty());
        let all = service
            .list("currencies", user_id, ReferenceListQuery { include_archived: true, ..list_query() })
            .await
            .unwrap();
        assert_eq!(all.content[0]["isArchived"], true);

        let err = service
            .create("accounts", user_id, json!({"code": "C", "name": "C", "currency": "USD", "accountType": "CARD"}))
            .await
            .unwrap_err();
        assert!(matches!(err, ReferenceError::Validation(_)));

        let restored = service.restore("currencies", user_id, "USD").await.unwrap();
        assert_eq!(restored["isArchived"], false);
    }

    #[tokio::test]
    async fn test_unknown_kind_and_missing_record() {
        let service = service();
        let user_id = Uuid::new_v4();

        assert!(matches!(
            service.get("planets", user_id, "X").await,
            Err(ReferenceError::UnknownKind(_))
        ));
        assert!(matches!(
            service.get("currencies", user_id, "X").await,
            Err(ReferenceError::NotFound { .. })
        ));
        assert!(matches!(
            service.archive("currencies", user_id, "X").await,
            Err(ReferenceError::NotFound { .. })
        ));
    }
}
