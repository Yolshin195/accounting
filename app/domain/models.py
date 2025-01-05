from dataclasses import dataclass
from datetime import datetime
from decimal import Decimal
from enum import Enum


@dataclass
class User:
    name: str


@dataclass
class TransactionTypeEnum(Enum):
    expense = 'expense'
    income = 'income'


@dataclass
class Reference:
    code: str
    name: str
    description: str | None


@dataclass
class Project(Reference):
    author: User
    users: list['ProjectUser']


@dataclass
class ProjectLink:
    project: Project


@dataclass
class Currency(Reference, ProjectLink):
    pass


@dataclass
class Category(Reference, ProjectLink):
    pass


@dataclass
class Account(Reference, ProjectLink):
    currency: Currency


@dataclass
class ProjectUser(ProjectLink):
    user: User
    account: Account


@dataclass
class Transaction(ProjectLink):
    created_at: datetime
    account: Account
    category: Category
    author: User
    value: Decimal
    description: str | None
    type: TransactionTypeEnum


@dataclass
class Expense(Transaction):
    """Class representing an expense transaction."""
    type: TransactionTypeEnum = TransactionTypeEnum.expense


@dataclass
class Income(Transaction):
    """Class representing an income transaction."""
    type: TransactionTypeEnum = TransactionTypeEnum.income
