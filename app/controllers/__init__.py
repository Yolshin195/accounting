from litestar import Router
from .reference import CategoryController, CurrencyController, AccountController
from .transaction import CreateTransactionController

router = Router(path='/api/v1/reference', tags=["V1"], route_handlers=[
    CategoryController,
    CurrencyController,
    AccountController,

    CreateTransactionController,
])

__all__ = [
    'router'
]
