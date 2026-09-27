use crate::application::dtos::currency_dto::{CreateCurrencyDto, CurrencyDto, UpdateCurrencyDto};
use crate::application::dtos::pagination_dto::PagedResponse;
use crate::application::dtos::reference_dto::{ensure_valid, ReferenceListQuery};
use crate::application::services::reference_service;
use crate::application::traits::currency_repo::CurrencyRepository;
use crate::domain::currency::Currency;
use crate::domain::reference::{ReferenceBase, ReferenceError};
use uuid::Uuid;

#[derive(Clone)]
pub struct CurrencyService<R: CurrencyRepository> {
    pub repo: R,
}

impl<R: CurrencyRepository> CurrencyService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn list(&self, user_id: Uuid, query: ReferenceListQuery) -> Result<PagedResponse<CurrencyDto>, ReferenceError> {
        reference_service::list_page(&self.repo, user_id, query, CurrencyDto::from).await
    }

    pub async fn get(&self, user_id: Uuid, code: &str) -> Result<CurrencyDto, ReferenceError> {
        reference_service::get_by_code(&self.repo, user_id, code).await.map(CurrencyDto::from)
    }

    pub async fn create(&self, user_id: Uuid, mut dto: CreateCurrencyDto) -> Result<CurrencyDto, ReferenceError> {
        let mut errors = Vec::new();
        dto.base.validate(&mut errors);
        dto.fields.validate(&mut errors);
        ensure_valid(errors)?;

        let currency = Currency {
            base: ReferenceBase {
                id: Uuid::new_v4(),
                user_id,
                code: dto.base.code,
                name: dto.base.name,
                description: dto.base.description,
                sort_order: dto.base.sort_order,
                is_archived: false,
            },
            symbol: dto.fields.symbol,
            decimals: dto.fields.decimals,
            is_base: dto.fields.is_base,
        };
        self.repo.insert(&currency).await?;
        Ok(currency.into())
    }

    pub async fn update(&self, user_id: Uuid, code: &str, mut dto: UpdateCurrencyDto) -> Result<CurrencyDto, ReferenceError> {
        let mut errors = Vec::new();
        dto.base.validate(&mut errors);
        dto.fields.validate(&mut errors);
        ensure_valid(errors)?;

        let mut currency: Currency = reference_service::get_by_code(&self.repo, user_id, code).await?;
        dto.base.apply_to(&mut currency.base);
        dto.fields.apply_to(&mut currency);
        self.repo.update(&currency).await?;
        Ok(currency.into())
    }

    pub async fn archive(&self, user_id: Uuid, code: &str) -> Result<(), ReferenceError> {
        reference_service::archive(&self.repo, user_id, code).await
    }

    pub async fn restore(&self, user_id: Uuid, code: &str) -> Result<CurrencyDto, ReferenceError> {
        reference_service::restore(&self.repo, user_id, code).await.map(CurrencyDto::from)
    }
}
