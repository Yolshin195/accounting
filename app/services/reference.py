from uuid import UUID

from litestar.exceptions import HTTPException
from litestar.plugins.sqlalchemy import (
    service,
    filters
)

from app.models import CategoryModel, ProjectUserModel, CurrencyModel, AccountModel
from app.reposiotories.reference import CategoryRepository, CurrencyRepository, AccountRepository
from app.schema import ReferenceDTO, CreateReferenceDTO


class CategoryService(service.SQLAlchemyAsyncRepositoryService[CategoryModel]):
    repository_type = CategoryRepository

    def __init__(self, *args, user_project: ProjectUserModel, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self.user_project = user_project

    async def get_all(self, limit_offset: filters.LimitOffset) -> service.OffsetPagination[ReferenceDTO]:
        """Список всех записей"""
        results, total = await self.list_and_count(
            limit_offset,
            CategoryModel.project_id == self.user_project.project_id,
        )
        return self.to_schema(
            data=results,
            total=total,
            filters=[limit_offset],
            schema_type=ReferenceDTO
        )

    async def get_by_id(self, item_id: UUID) -> ReferenceDTO:
        result = await self.get_one_or_none(
            CategoryModel.id == item_id,
            CategoryModel.project_id == self.user_project.project_id,
        )
        if result is None:
            raise HTTPException(status_code=404, detail="Item not found")

        return self.to_schema(data=result, schema_type=ReferenceDTO)

    async def add(self, data: CreateReferenceDTO) -> None:
        model = await self.to_model(data.__dict__)
        model.project_id = self.user_project.project_id
        await self.create(model, auto_commit=True)


class CurrencyService(service.SQLAlchemyAsyncRepositoryService[CurrencyModel]):
    repository_type = CurrencyRepository

    def __init__(self, *args, user_project: ProjectUserModel, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self.user_project = user_project

    async def get_all(self, limit_offset: filters.LimitOffset) -> service.OffsetPagination[ReferenceDTO]:
        """Список всех записей"""
        results, total = await self.list_and_count(
            limit_offset,
            CurrencyModel.project_id == self.user_project.project_id,
        )
        return self.to_schema(
            data=results,
            total=total,
            filters=[limit_offset],
            schema_type=ReferenceDTO
        )

    async def get_by_id(self, item_id: UUID) -> ReferenceDTO:
        """Получить запись по ID"""
        result = await self.get_one_or_none(
            CurrencyModel.id == item_id,
            CurrencyModel.project_id == self.user_project.project_id,
        )
        if result is None:
            raise HTTPException(status_code=404, detail="Item not found")

        return self.to_schema(data=result, schema_type=ReferenceDTO)

    async def add(self, data: CreateReferenceDTO) -> None:
        """Добавить новую запись"""
        model = await self.to_model(data.__dict__)
        model.project_id = self.user_project.project_id
        await self.create(model, auto_commit=True)


class AccountService(service.SQLAlchemyAsyncRepositoryService[AccountModel]):
    repository_type = AccountRepository

    def __init__(self, *args, user_project: ProjectUserModel, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self.user_project = user_project

    async def get_all(self, limit_offset: filters.LimitOffset) -> service.OffsetPagination[ReferenceDTO]:
        """Список всех записей"""
        results, total = await self.list_and_count(
            limit_offset,
            AccountModel.project_id == self.user_project.project_id,
        )
        return self.to_schema(
            data=results,
            total=total,
            filters=[limit_offset],
            schema_type=ReferenceDTO
        )

    async def get_by_id(self, item_id: UUID) -> ReferenceDTO:
        """Получить запись по ID"""
        result = await self.get_one_or_none(
            AccountModel.id == item_id,
            AccountModel.project_id == self.user_project.project_id,
        )
        if result is None:
            raise HTTPException(status_code=404, detail="Item not found")

        return self.to_schema(data=result, schema_type=ReferenceDTO)

    async def add(self, data: CreateReferenceDTO) -> None:
        """Добавить новую запись"""
        model = await self.to_model(data.__dict__)
        model.project_id = self.user_project.project_id
        await self.create(model, auto_commit=True)


