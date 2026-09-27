use crate::application::traits::reference_repo::ReferenceRepository;
use crate::domain::currency::Currency;
use crate::domain::reference::ReferenceError;
use async_trait::async_trait;

#[async_trait]
pub trait CurrencyRepository: ReferenceRepository<Currency> {
    async fn insert(&self, currency: &Currency) -> Result<(), ReferenceError>;
    async fn update(&self, currency: &Currency) -> Result<(), ReferenceError>;
}
