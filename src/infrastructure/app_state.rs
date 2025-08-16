use crate::application::services::auth_service::AuthService;
use crate::application::services::category_service::CategoryService;
use crate::application::services::transaction_service::TransactionService;
use crate::infrastructure::db::postgres_category_repository::PostgresCategoryRepo;
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
