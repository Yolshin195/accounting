from dataclasses import dataclass
from uuid import UUID

from pydantic import BaseModel


class ReferenceDTO(BaseModel):
    id: UUID
    code: str
    name: str
    description: str | None

    model_config = {"from_attributes": True}


class CreateReferenceDTO(BaseModel):
    code: str
    name: str
    description: str | None


class UpdateReferenceDTO(BaseModel):
    code: str
    name: str
    description: str | None


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

@dataclass
class ReferenceBase:
    id: UUID
    code: str
    name: str
    description: str | None


@dataclass
class CreateReferenceBase:
    code: str
    name: str
    description: str | None
