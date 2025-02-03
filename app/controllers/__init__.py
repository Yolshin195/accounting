from litestar import Router
from .reference import CategoryController, CurrencyController

router = Router(path='/api/v1/', route_handlers=[
    CategoryController, CurrencyController
])

__all__ = [
    'router'
]
