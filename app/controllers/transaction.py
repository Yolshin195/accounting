from litestar import Controller, post
from litestar.di import Provide

from app.providers import get_create_transaction_service
from app.schema import CreateTransactionDTO
from app.services.transaction import CreateTransactionService


class CreateTransactionController(Controller):
    path = "transaction"
    tags = ["Transaction"]

    dependencies = {
        "service": Provide(get_create_transaction_service)
    }

    @post("/expense")
    async def create_expense(
            self,
            data: CreateTransactionDTO,
            service: CreateTransactionService
    ) -> None:
        """Создать новую запись"""
        await service.expense(data)

    @post("/income")
    async def create_income(
            self,
            data: CreateTransactionDTO,
            service: CreateTransactionService
    ) -> None:
        await service.income(data)
