from litestar import Router
from .reference import CategoryController


router = Router(path='/api/v1/', route_handlers=[
    CategoryController
])

__all__ = [
    'router'
]
