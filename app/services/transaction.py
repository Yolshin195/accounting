from litestar.plugins.sqlalchemy import (
    service,
)

from app.models import TransactionModel, LedgerEntryModel, ProjectUserModel, TransactionTypeEnum, \
    TransactionTypeValueEnum
from app.reposiotories import TransactionRepository, LedgerEntryRepository
from app.schema import CreateTransactionDTO


class TransactionRepositoryService(service.SQLAlchemyAsyncRepositoryService[TransactionModel]):
    """Expense repository service."""
    repository_type = TransactionRepository


class LedgerEntryRepositoryService(service.SQLAlchemyAsyncRepositoryService[LedgerEntryModel]):
    """Income repository service."""
    repository_type = LedgerEntryRepository


class GetAllExpenseTransactionsService:
    """Get all expenses transactions service."""
    def __init__(self, user_project: ProjectUserModel, transaction_repository: TransactionRepositoryService):
        self.user_project = user_project
        self.transaction_repository = transaction_repository

    def execute(self):
        pass


class CreateTransactionService:
    """Create expense transaction service."""
    def __init__(
            self,
            user_project: ProjectUserModel,
            transaction_repository: TransactionRepositoryService,
            ledger_repository: LedgerEntryRepositoryService,
    ):
        self.user_project = user_project
        self.transaction_repository = transaction_repository
        self.ledger_repository = ledger_repository

    async def expense(self, data: CreateTransactionDTO):
        """Создать расходную транзакцию с учетом двойной записи."""

        account_id = data.account_id or self.user_project.account_id
        if account_id is None:
            raise ValueError(
                "Account ID не задан. Пожалуйста, укажите account_id "
                "либо в параметрах транзакции, "
                "либо в проекте пользователя."
            )

        # Создание главной записи в системе (Ledger Entry)
        ledger_entry = LedgerEntryModel(
            project_id=self.user_project.project_id
        )
        await self.ledger_repository.create(ledger_entry)

        # Создание дебетовой транзакции
        debit_transaction = TransactionModel(
            type=TransactionTypeEnum.debit,
            value=data.value,
            value_type=TransactionTypeValueEnum.expense,
            description=data.description,
            account_id=None,
            category_id=data.category_id,
            ledger_entry_id=ledger_entry.id,
            author_id=self.user_project.user_id,
            project_id=self.user_project.project_id
        )
        await self.transaction_repository.create(debit_transaction)

        # Создание кредитовой транзакции (корреспондирующий счет, например, "Банк")
        credit_transaction = TransactionModel(
            type=TransactionTypeEnum.credit,
            value=data.value,
            value_type=TransactionTypeValueEnum.expense,
            description=data.description,
            account_id=account_id,
            category_id=data.category_id,
            ledger_entry_id=ledger_entry.id,
            author_id=self.user_project.user_id,
            project_id=self.user_project.project_id
        )
        await self.transaction_repository.create(credit_transaction)

        await self.transaction_repository.repository.session.commit()

    async def income(self, data: CreateTransactionDTO):
        account_id = data.account_id or self.user_project.account_id
        if account_id is None:
            raise ValueError(
                "Account ID не задан. Пожалуйста, укажите account_id "
                "либо в параметрах транзакции, "
                "либо в проекте пользователя."
            )

        # Создание главной записи в системе (Ledger Entry)
        ledger_entry = LedgerEntryModel(
            project_id=self.user_project.project_id
        )
        await self.ledger_repository.create(ledger_entry)

        # Создание дебетовой транзакции
        debit_transaction = TransactionModel(
            type=TransactionTypeEnum.debit,
            value=data.value,
            value_type=TransactionTypeValueEnum.income,
            description=data.description,
            account_id=account_id,
            category_id=data.category_id,
            ledger_entry_id=ledger_entry.id,
            author_id=self.user_project.user_id,
            project_id=self.user_project.project_id
        )
        await self.transaction_repository.create(debit_transaction)

        # Создание кредитовой транзакции (корреспондирующий счет, например, "Банк")
        credit_transaction = TransactionModel(
            type=TransactionTypeEnum.credit,
            value=data.value,
            value_type=TransactionTypeValueEnum.income,
            description=data.description,
            account_id=None,
            category_id=data.category_id,
            ledger_entry_id=ledger_entry.id,
            author_id=self.user_project.user_id,
            project_id=self.user_project.project_id
        )
        await self.transaction_repository.create(credit_transaction)

        await self.transaction_repository.repository.session.commit()
