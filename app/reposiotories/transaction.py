from litestar.plugins.sqlalchemy import (
    repository,
)

from app.models import ExpenseModel, IncomeModel


class ExpenseRepository(repository.SQLAlchemyAsyncRepository[ExpenseModel]):
    """Expense repository."""

    model_type = ExpenseModel


class IncomeRepository(repository.SQLAlchemyAsyncRepository[IncomeModel]):
    """Income repository."""

    model_type = IncomeModel
