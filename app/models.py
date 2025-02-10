from sqlalchemy.orm import Mapped, mapped_column, relationship, declared_attr

from litestar.contrib.sqlalchemy.base import UUIDAuditBase
from decimal import Decimal
import uuid
from enum import Enum
from typing import List, Optional
from sqlalchemy import String, ForeignKey, DateTime, Numeric, Enum as SAEnum

from litestar_users.adapter.sqlalchemy.mixins import SQLAlchemyUserMixin

class TransactionTypeEnum(Enum):
    debit = "debit"
    credit = "credit"


class TransactionTypeValueEnum(Enum):
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
    """
    Cущность хранит информацию о валютах
    Пример:
    - Рубли
    - Баты
    - Доллары
    """
    __tablename__ = "currencies"


class CategoryModel(BaseModel, ReferenceMixin, ProjectLinkMixin):
    __tablename__ = "categories"


class AccountModel(BaseModel, ReferenceMixin, ProjectLinkMixin):
    """
    Сущность хранит информацию о счетах в проекте
    пример:
    - Кеш рубли
    - Кеш баты
    - Банковская карта баты
    """
    __tablename__ = "accounts"
    currency_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("currencies.id"), nullable=False)
    currency: Mapped[CurrencyModel] = relationship()


class ProjectUserModel(BaseModel, ProjectLinkMixin):
    __tablename__ = "project_users"
    current: Mapped[bool] = mapped_column(default=False, nullable=False)

    user_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("users.id"), nullable=False)
    user: Mapped[UserModel] = relationship(back_populates="project_users")

    account_id: Mapped[uuid.UUID | None] = mapped_column(ForeignKey("accounts.id"), default=None, nullable=True)
    account: Mapped[AccountModel] = relationship()


class TransactionModel(BaseModel, ProjectLinkMixin):
    __tablename__ = "transactions"
    type: Mapped[TransactionTypeEnum] = mapped_column(
        SAEnum(TransactionTypeEnum, native_enum=False), nullable=False
    )
    value: Mapped[Decimal] = mapped_column(Numeric(10, 2), nullable=False)
    value_type: Mapped[TransactionTypeValueEnum] = mapped_column(
        SAEnum(TransactionTypeValueEnum, native_enum=False), nullable=False
    )
    description: Mapped[str | None] = mapped_column(String, nullable=True)

    account_id: Mapped[uuid.UUID | None] = mapped_column(ForeignKey("accounts.id"), nullable=True)
    account: Mapped[AccountModel | None] = relationship()

    category_id: Mapped[uuid.UUID | None] = mapped_column(ForeignKey("categories.id"), nullable=True)
    category: Mapped[CategoryModel | None] = relationship()

    ledger_entry_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("ledger_entries.id"), nullable=False)
    ledger_entry: Mapped["LedgerEntryModel"] = relationship(back_populates="transactions")

    author_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("users.id"), nullable=False)
    author: Mapped[UserModel] = relationship()


class LedgerEntryModel(BaseModel, ProjectLinkMixin):
    """Запись в журнале (группирует две транзакции)"""
    __tablename__ = "ledger_entries"

    transactions: Mapped[list["TransactionModel"]] = relationship(back_populates="ledger_entry")
