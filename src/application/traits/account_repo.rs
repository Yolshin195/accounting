use crate::application::traits::reference_repo::ReferenceRepository;
use crate::domain::account::Account;
use crate::domain::reference::ReferenceError;
use async_trait::async_trait;

#[async_trait]
pub trait AccountRepository: ReferenceRepository<Account> {
    async fn insert(&self, account: &Account) -> Result<(), ReferenceError>;
    async fn update(&self, account: &Account) -> Result<(), ReferenceError>;
}
