from dataclasses import dataclass
from uuid import UUID


@dataclass
class User:
    id: UUID
    username: str
    email: str


@dataclass
class CreateUser:
    username: str
    email: str
    password: str


class LoginUser:
    username: str
    password: str
