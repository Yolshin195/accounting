use crate::application::dtos::account_dto::{AccountDto, CreateAccountDto, UpdateAccountDto};
use crate::application::dtos::pagination_dto::PagedResponse;
use crate::application::dtos::reference_dto::{ensure_valid, ReferenceListQuery};
use crate::application::services::reference_service;
use crate::application::traits::account_repo::AccountRepository;
use crate::application::traits::currency_repo::CurrencyRepository;
use crate::domain::account::Account;
use crate::domain::reference::{ReferenceBase, ReferenceError};
use uuid::Uuid;

#[derive(Clone)]
pub struct AccountService<AR: AccountRepository, CR: CurrencyRepository> {
    pub repo: AR,
    pub currency_repo: CR,
}

impl<AR: AccountRepository, CR: CurrencyRepository> AccountService<AR, CR> {
    pub fn new(repo: AR, currency_repo: CR) -> Self {
        Self { repo, currency_repo }
    }

    pub async fn list(&self, user_id: Uuid, query: ReferenceListQuery) -> Result<PagedResponse<AccountDto>, ReferenceError> {
        reference_service::list_page(&self.repo, user_id, query, AccountDto::from).await
    }

    pub async fn get(&self, user_id: Uuid, code: &str) -> Result<AccountDto, ReferenceError> {
        reference_service::get_by_code(&self.repo, user_id, code).await.map(AccountDto::from)
    }

    pub async fn create(&self, user_id: Uuid, mut dto: CreateAccountDto) -> Result<AccountDto, ReferenceError> {
        let mut errors = Vec::new();
        dto.base.validate(&mut errors);
        dto.fields.validate(&mut errors);
        ensure_valid(errors)?;
        let currency_id =
            reference_service::resolve_code(&self.currency_repo, user_id, "currency", &dto.fields.currency).await?;

        let account = Account {
            base: ReferenceBase {
                id: Uuid::new_v4(),
                user_id,
                code: dto.base.code,
                name: dto.base.name,
                description: dto.base.description,
                sort_order: dto.base.sort_order,
                is_archived: false,
            },
            currency_id,
            currency_code: dto.fields.currency,
            account_type: dto.fields.account_type,
            initial_balance: dto.fields.initial_balance,
        };
        self.repo.insert(&account).await?;
        Ok(account.into())
    }

    pub async fn update(&self, user_id: Uuid, code: &str, mut dto: UpdateAccountDto) -> Result<AccountDto, ReferenceError> {
        let mut errors = Vec::new();
        dto.base.validate(&mut errors);
        dto.fields.validate(&mut errors);
        ensure_valid(errors)?;

        let mut account: Account = reference_service::get_by_code(&self.repo, user_id, code).await?;
        if account.currency_code != dto.fields.currency {
            account.currency_id =
                reference_service::resolve_code(&self.currency_repo, user_id, "currency", &dto.fields.currency).await?;
            account.currency_code = dto.fields.currency;
        }
        dto.base.apply_to(&mut account.base);
        account.account_type = dto.fields.account_type;
        account.initial_balance = dto.fields.initial_balance;
        self.repo.update(&account).await?;
        Ok(account.into())
    }

    pub async fn archive(&self, user_id: Uuid, code: &str) -> Result<(), ReferenceError> {
        reference_service::archive(&self.repo, user_id, code).await
    }

    pub async fn restore(&self, user_id: Uuid, code: &str) -> Result<AccountDto, ReferenceError> {
        reference_service::restore(&self.repo, user_id, code).await.map(AccountDto::from)
    }
}
