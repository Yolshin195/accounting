use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use crate::application::dtos::category_dto::{CategoryDto, CreateCategoryDto};
use crate::application::dtos::pagination_dto::{PageInfo, Pagination, PagedResponse};
use crate::application::dtos::transaction_dto::{
    CategoryExpenseSummaryDto, CreateTransactionDto, TransactionDto, UpdateTransactionDto,
};
use crate::application::dtos::user_dto::{
    CreateUserDto, JwtResponse, LoginRequest, LoginTelegramBotDto,
};
use crate::application::dtos::account_dto::{AccountDto, AccountFields, CreateAccountDto, UpdateAccountDto};
use crate::application::dtos::currency_dto::{CreateCurrencyDto, CurrencyDto, CurrencyFields, UpdateCurrencyDto};
use crate::application::dtos::reference_dto::{
    CreateReferenceBaseDto, ErrorResponse, ReferenceBaseDto, UpdateReferenceBaseDto,
};
use crate::domain::account::AccountType;
use crate::domain::reference::FieldError;
use crate::interface::http::handlers::{
    account_handler, category_handler, currency_handler, transaction_handler, user_handler,
};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Accounting API",
        version = "0.2.6",
        description = "Система управления личными финансами с возможностью отслеживания доходов и расходов."
    ),
    paths(
        user_handler::register,
        user_handler::login,
        user_handler::login_telegram,
        category_handler::create_category,
        category_handler::list_categories,
        category_handler::delete_category_by_code,
        transaction_handler::transaction_list,
        transaction_handler::create_income_transaction,
        transaction_handler::create_expense_transaction,
        transaction_handler::delete_transaction,
        transaction_handler::find_all_transaction_by_month,
        transaction_handler::update_transaction,
        transaction_handler::find_transaction_by_id,
        transaction_handler::sum_today_expenses_grouped_by_category,
        currency_handler::list_currencies,
        currency_handler::get_currency,
        currency_handler::create_currency,
        currency_handler::update_currency,
        currency_handler::archive_currency,
        currency_handler::restore_currency,
        account_handler::list_accounts,
        account_handler::get_account,
        account_handler::create_account,
        account_handler::update_account,
        account_handler::archive_account,
        account_handler::restore_account,
    ),
    components(schemas(
        CreateUserDto,
        LoginRequest,
        LoginTelegramBotDto,
        JwtResponse,
        CategoryDto,
        CreateCategoryDto,
        TransactionDto,
        CreateTransactionDto,
        UpdateTransactionDto,
        CategoryExpenseSummaryDto,
        Pagination,
        PageInfo,
        PagedResponse<CategoryDto>,
        PagedResponse<TransactionDto>,
        ReferenceBaseDto,
        CreateReferenceBaseDto,
        UpdateReferenceBaseDto,
        ErrorResponse,
        FieldError,
        CurrencyFields,
        CurrencyDto,
        CreateCurrencyDto,
        UpdateCurrencyDto,
        PagedResponse<CurrencyDto>,
        AccountType,
        AccountFields,
        AccountDto,
        CreateAccountDto,
        UpdateAccountDto,
        PagedResponse<AccountDto>
    )),
    tags(
        (name = "Users", description = "Регистрация и аутентификация пользователей"),
        (name = "Categories", description = "Управление категориями доходов/расходов"),
        (name = "Transactions", description = "Управление транзакциями"),
        (name = "References", description = "Справочники: у всех одинаковые ручки и базовые поля (ReferenceBaseDto)"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
