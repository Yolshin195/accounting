from litestar import Router
from .user import UserController


router = Router(path='/api/v1/', route_handlers=[
    UserController
])

__all__ = [
    'router'
]