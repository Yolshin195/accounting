from litestar.plugins.sqlalchemy import (
    repository,
)

from app.models import CategoryModel, AccountModel, CurrencyModel


class CategoryRepository(repository.SQLAlchemyAsyncRepository[CategoryModel]):
    """Category repository."""

    model_type = CategoryModel


class AccountRepository(repository.SQLAlchemyAsyncRepository[AccountModel]):
    """Account repository."""

    model_type = AccountModel


class CurrencyRepository(repository.SQLAlchemyAsyncRepository[CurrencyModel]):
    """Currency repository."""

    model_type = CurrencyModel
