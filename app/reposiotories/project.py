from litestar.plugins.sqlalchemy import (
    repository,
)

from app.models import ProjectModel, ProjectUserModel


class ProjectRepository(repository.SQLAlchemyAsyncRepository[ProjectModel]):
    """Project repository."""

    model_type = ProjectModel


class ProjectUserRepository(repository.SQLAlchemyAsyncRepository[ProjectUserModel]):
    """ProjectUser repository."""

    model_type = ProjectUserModel
