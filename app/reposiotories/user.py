from litestar.plugins.sqlalchemy import (
    repository,
)

from app.models import UserModel


class UserRepository(repository.SQLAlchemyAsyncRepository[UserModel]):
    """Author repository."""

    model_type = UserModel
