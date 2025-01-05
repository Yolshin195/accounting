from dataclasses import dataclass
from typing import Any
from advanced_alchemy.extensions.litestar.dto import SQLAlchemyDTO, SQLAlchemyDTOConfig
from litestar import Request

from litestar.dto import DataclassDTO
from litestar.middleware.session.server_side import ServerSideSessionConfig
from litestar.security.jwt import JWTAuth

from litestar_users import LitestarUsersPlugin, LitestarUsersConfig
from litestar_users.config import (
    AuthHandlerConfig,
    RegisterHandlerConfig,
    VerificationHandlerConfig,
)
from litestar_users.service import BaseUserService

from app.models import UserModel as User

ENCODING_SECRET = "1234567890abcdef"  # noqa: S105
DATABASE_URL = "sqlite+aiosqlite:///"


@dataclass
class UserRegistrationSchema:
    email: str
    password: str


class UserRegistrationDTO(DataclassDTO[UserRegistrationSchema]):
    """User registration DTO."""


class UserReadDTO(SQLAlchemyDTO[User]):
    config = SQLAlchemyDTOConfig(exclude={"password_hash", "projects", "project_users"})


class UserUpdateDTO(SQLAlchemyDTO[User]):
    config = SQLAlchemyDTOConfig(exclude={"password_hash", "projects", "project_users"}, partial=True)


class UserService(BaseUserService[User, Any]):  # type: ignore[type-var]
    async def post_registration_hook(self, user: User, request: Request | None = None) -> None:
        print(f"User <{user.email}> has registered!")
        await self.user_repository.session.commit()


litestar_users = LitestarUsersPlugin(
    config=LitestarUsersConfig(
        auth_backend_class=JWTAuth,
        secret=ENCODING_SECRET,
        user_model=User,  # pyright: ignore
        user_read_dto=UserReadDTO,
        user_registration_dto=UserRegistrationDTO,
        user_update_dto=UserUpdateDTO,
        user_service_class=UserService,  # pyright: ignore
        auto_commit_transactions=False,
        auth_handler_config=AuthHandlerConfig(),
        register_handler_config=RegisterHandlerConfig(),
        verification_handler_config=VerificationHandlerConfig(),
        session_backend_config=ServerSideSessionConfig()
    )
)
