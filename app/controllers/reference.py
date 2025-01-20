from uuid import UUID

from advanced_alchemy.service import OffsetPagination
from litestar import Controller, get, post, put, delete
from litestar.di import Provide
from litestar.plugins.sqlalchemy import (
    filters,
)

from app.providers import get_category_service
from app.schema import ReferenceDTO, CreateReferenceDTO, UpdateReferenceDTO
from app.services.reference import CategoryService


class CategoryController(Controller):
    path = "/reference"
    dependencies = {
        "service": Provide(get_category_service)
    }

    @get("/")
    async def category_list(
            self,
            limit_offset: filters.LimitOffset,
            service: CategoryService
    ) -> OffsetPagination[ReferenceDTO]:
        """Список всех записей"""
        return await service.get_all(limit_offset)

    @get("/{item_id:uuid}")
    async def get_category(
            self,
            item_id: UUID,
            service: CategoryService
    ) -> ReferenceDTO:
        """Получить запись по UUID"""
        return await service.get_by_id(item_id)

    @post("/")
    async def create_category(
            self,
            data: CreateReferenceDTO,
            service: CategoryService
    ) -> None:
        """Создать новую запись"""
        await service.add(data)

    @put("/{item_id:uuid}")
    async def update_category(
            self,
            item_id: UUID,
            data: UpdateReferenceDTO,
            service: CategoryService
    ) -> None:
        """Обновить данные записи"""
        await service.update(
            data.model_dump(exclude_unset=True, exclude_none=True),
            item_id=item_id,
            auto_commit=True
        )

    @delete("/{item_id:uuid}")
    async def delete_category(
            self,
            item_id: UUID,
            service: CategoryService
    ) -> None:
        """Удалить запись по UUID"""
        await service.delete(item_id=item_id)
