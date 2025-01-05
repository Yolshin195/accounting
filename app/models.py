from sqlalchemy.orm import Mapped, mapped_column, relationship, declared_attr

from litestar.contrib.sqlalchemy.base import UUIDAuditBase
from datetime import datetime
from decimal import Decimal
import uuid
from enum import Enum
from typing import List, Optional
from sqlalchemy import String, ForeignKey, DateTime, Numeric, Enum as SAEnum

from litestar_users.adapter.sqlalchemy.mixins import SQLAlchemyUserMixin

class TransactionTypeEnum(Enum):
    expense = "expense"
    income = "income"


class BaseModel(UUIDAuditBase):
    __abstract__ = True


class UserModel(BaseModel, SQLAlchemyUserMixin):
    __tablename__ = "users"
    username: Mapped[str | None] =  mapped_column(default=None, nullable=True)

    projects: Mapped[List["ProjectModel"]] = relationship(back_populates="author")
    project_users: Mapped[List["ProjectUserModel"]] = relationship(back_populates="user")


class ReferenceMixin:
    __abstract__ = True
    code: Mapped[str] = mapped_column(String, nullable=False)
    name: Mapped[str] = mapped_column(String, nullable=False)
    description: Mapped[Optional[str]] = mapped_column(String)


class ProjectModel(BaseModel, ReferenceMixin):
    __tablename__ = "projects"
    default: Mapped[bool] = mapped_column(String, nullable=False)
    author_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("users.id"), nullable=False)
    author: Mapped[UserModel] = relationship(back_populates="projects")
    users: Mapped[List["ProjectUserModel"]] = relationship(back_populates="project")


class ProjectLinkMixin:
    __abstract__ = True
    project_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("projects.id"), nullable=False)

    @declared_attr
    def project(cls) -> Mapped["ProjectModel"]:
        return relationship()


class CurrencyModel(BaseModel, ReferenceMixin, ProjectLinkMixin):
    __tablename__ = "currencies"


class CategoryModel(BaseModel, ReferenceMixin, ProjectLinkMixin):
    __tablename__ = "categories"


class AccountModel(BaseModel, ReferenceMixin, ProjectLinkMixin):
    __tablename__ = "accounts"
    currency_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("currencies.id"), nullable=False)
    currency: Mapped[CurrencyModel] = relationship()


class ProjectUserModel(BaseModel, ProjectLinkMixin):
    __tablename__ = "project_users"
    user_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("users.id"), nullable=False)
    account_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("accounts.id"), nullable=False)
    user: Mapped[UserModel] = relationship(back_populates="project_users")
    account: Mapped[AccountModel] = relationship()


class TransactionModel(BaseModel, ProjectLinkMixin):
    __abstract__ = True
    created_at: Mapped[datetime] = mapped_column(DateTime, default=datetime.utcnow, nullable=False)
    account_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("accounts.id"), nullable=False)
    category_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("categories.id"), nullable=False)
    author_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("users.id"), nullable=False)
    value: Mapped[Decimal] = mapped_column(Numeric(10, 2), nullable=False)
    description: Mapped[Optional[str]] = mapped_column(String)

    @declared_attr
    def account(self) -> Mapped[AccountModel]:
        return relationship()

    @declared_attr
    def category(self) -> Mapped[CategoryModel]:
        return relationship()

    @declared_attr
    def author(self) -> Mapped[UserModel]:
        return relationship()


class ExpenseModel(TransactionModel):
    __tablename__ = "expenses"


class IncomeModel(TransactionModel):
    __tablename__ = "incomes"
