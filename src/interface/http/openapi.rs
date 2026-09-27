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
use crate::application::dtos::reference_dto::{ErrorResponse, ReferenceItem};
use crate::domain::reference::{EnumOption, FieldDescriptor, FieldError, FieldType, ReferenceDescriptor};
use crate::interface::http::handlers::{
    category_handler, reference_handler, transaction_handler, user_handler,
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
        reference_handler::list_reference_descriptors,
        reference_handler::list_references,
        reference_handler::get_reference,
        reference_handler::create_reference,
        reference_handler::update_reference,
        reference_handler::archive_reference,
        reference_handler::restore_reference,
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
        ReferenceDescriptor,
        FieldDescriptor,
        FieldType,
        EnumOption,
        ReferenceItem,
        PagedResponse<ReferenceItem>,
        ErrorResponse,
        FieldError
    )),
    tags(
        (name = "Users", description = "Регистрация и аутентификация пользователей"),
        (name = "Categories", description = "Управление категориями доходов/расходов"),
        (name = "Transactions", description = "Управление транзакциями"),
        (name = "References", description = "Справочники (валюты, счета, ...) с единым интерфейсом"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
