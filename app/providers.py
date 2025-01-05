from sqlalchemy.ext.asyncio import AsyncSession

from app.services.reference import CategoryService
from app.services.user import UserService


async def get_user_service(db_session: AsyncSession):
    return UserService(session=db_session)


async def get_category_service(db_session: AsyncSession):
    return CategoryService(session=db_session)
