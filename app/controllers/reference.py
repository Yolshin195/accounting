from typing import Any, TypeVar, Generic
from uuid import UUID

from advanced_alchemy.repository import SQLAlchemyAsyncRepository
from advanced_alchemy.service import OffsetPagination
from litestar import Controller, get, post, put, delete
from litestar.di import Provide
from sqlalchemy.ext.asyncio import AsyncSession
from litestar.plugins.sqlalchemy import (
    filters,
)
from app.providers import get_category_service
from app.services.reference import CategoryService


class CategoryController(Controller):
    dependencies = {"service": Provide(get_category_service)}

    @get("/")
    async def list(self, limit_offset: filters.LimitOffset, service: CategoryService) -> OffsetPagination[]:
        """Список всех записей"""
        results, total = service.list_and_count(limit_offset)
        service.to_schema(results)
        return OffsetPagination(
            items=results,
            total=total
        )

    @get("/{item_id:uuid}")
    async def get(self, item_id: UUID) -> T | None:
        """Получить запись по UUID"""
        return await self.repo.get(item_id)

    @post("/")
    async def create(self, data: dict[str, Any]) -> T:
        """Создать новую запись"""
        return await self.repo.add(data)

    @put("/{item_id:uuid}")
    async def update(self, item_id: UUID, data: dict[str, Any]) -> T:
        """Обновить данные записи"""
        return await self.repo.update(item_id, data)

    @delete("/{item_id:uuid}")
    async def delete(self, item_id: UUID) -> dict[str, str]:
        """Удалить запись по UUID"""
        await self.repo.delete(item_id)
        return {"message": "Record deleted"}
