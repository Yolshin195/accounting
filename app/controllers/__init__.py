from litestar import Router
from .reference import CategoryController, CurrencyController, AccountController

router = Router(path='/api/v1/reference', tags=["Reference"], route_handlers=[
    CategoryController, CurrencyController, AccountController
])

__all__ = [
    'router'
]
