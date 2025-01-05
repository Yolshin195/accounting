from litestar.plugins.sqlalchemy import (
    service,
)

from app.models import UserModel
from app.reposiotories import UserRepository
from app.schema import CreateUser, User


class UserService(service.SQLAlchemyAsyncRepositoryService[UserModel]):
    repository_type = UserRepository

    async def sign_up(self, create_user: CreateUser) -> User:
        user = UserModel(
            email=create_user.email,
            username=create_user.username,
            hash_password=create_user.password
        )
        user = await self.create(user)
        return User(
            id=user.id,
            email=user.email,
            username=user.username,
        )

