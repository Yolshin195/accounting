from typing import Annotated

from litestar.controller import Controller
from litestar import post
from litestar.di import Provide
from litestar.params import Body

from app.providers import get_user_service
from app.schema import CreateUser, User, LoginUser
from app.services.user import UserService


class UserController(Controller):
    dependencies = {"user_service": Provide(get_user_service)}

    @post('/signup')
    async def sign_up(
        self,
        data: Annotated[CreateUser, Body(title="Create User", description="Create a new user.")],
        user_service: UserService
    ) -> User:
        return await user_service.sign_up(data)

    @post('/login')
    async def login(self, data: Annotated[LoginUser, Body(title="Login", description="Login")], user_service: UserService) -> None:
        print(data, 'Test!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!')
