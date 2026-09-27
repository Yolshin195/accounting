use crate::application::services::account_service::AccountService;
use crate::application::services::auth_service::AuthService;
use crate::application::services::category_service::CategoryService;
use crate::application::services::currency_service::CurrencyService;
use crate::application::services::transaction_service::TransactionService;
use crate::infrastructure::db::postgres_account_repository::PostgresAccountRepository;
use crate::infrastructure::db::postgres_category_repository::PostgresCategoryRepo;
use crate::infrastructure::db::postgres_currency_repository::PostgresCurrencyRepository;
use crate::infrastructure::db::postgres_transaction_repository::PostgresTransactionRepo;
use crate::infrastructure::db::postgres_user_repository::PostgresUserRepository;

#[derive(Clone)]
pub struct CategoryAppState {
    pub category_service: CategoryService<PostgresCategoryRepo>,
}

#[derive(Clone)]
pub struct UserAppState {
    pub auth_service: AuthService<PostgresUserRepository>,
}

#[derive(Clone)]
pub struct TransactionAppState {
    pub transaction_service: TransactionService<PostgresTransactionRepo, PostgresCategoryRepo>
}

#[derive(Clone)]
pub struct CurrencyAppState {
    pub currency_service: CurrencyService<PostgresCurrencyRepository>,
}

#[derive(Clone)]
pub struct AccountAppState {
    pub account_service: AccountService<PostgresAccountRepository, PostgresCurrencyRepository>,
}
