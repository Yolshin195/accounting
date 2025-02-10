from litestar.plugins.sqlalchemy import (
    repository
)

from app.models import TransactionModel, LedgerEntryModel


class TransactionRepository(repository.SQLAlchemyAsyncRepository[TransactionModel]):
    """Transaction repository."""

    model_type = TransactionModel


class LedgerEntryRepository(repository.SQLAlchemyAsyncRepository[LedgerEntryModel]):
    """Ledger Entry repository."""

    model_type = LedgerEntryModel
