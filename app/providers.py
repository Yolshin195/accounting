from typing import Any

from litestar import Request
from sqlalchemy.ext.asyncio import AsyncSession

from app.models import ProjectUserModel, UserModel
from app.reposiotories.project import ProjectUserRepository
from app.services.project import GetUserProjectService
from app.services.reference import CategoryService, CurrencyService
from app.services.user import UserService


async def get_user_service(db_session: AsyncSession):
    return UserService(session=db_session)


async def get_user_project(request: Request[UserModel, Any, Any], db_session: AsyncSession) -> ProjectUserModel:
    return await GetUserProjectService(repo=await get_project_user_repository(db_session), user=request.user).execute()


async def get_category_service(request: Request[UserModel, Any, Any], db_session: AsyncSession):
    user_project = await get_user_project(request, db_session)
    return CategoryService(session=db_session, user_project=user_project)


async def get_currency_service(request: Request[UserModel, Any, Any], db_session: AsyncSession):
    """Провайдер для CurrencyService."""
    user_project = await get_user_project(request, db_session)
    return CurrencyService(session=db_session, user_project=user_project)


async def get_project_user_repository(db_session: AsyncSession) -> ProjectUserRepository:
    return ProjectUserRepository(session=db_session)
