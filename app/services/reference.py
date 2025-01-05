from litestar.plugins.sqlalchemy import (
    service,
)

from app.models import CategoryModel
from app.reposiotories.reference import CategoryRepository


class CategoryService(service.SQLAlchemyAsyncRepositoryService[CategoryModel]):
    repository_type = CategoryRepository
