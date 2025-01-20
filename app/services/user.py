from litestar.plugins.sqlalchemy import (
    service,
)

from app.models import UserModel, ProjectModel, ProjectUserModel
from app.reposiotories import UserRepository


class UserService(service.SQLAlchemyAsyncRepositoryService[UserModel]):
    repository_type = UserRepository

    async def init_new_user(self, user: UserModel, auto_commit: bool | None = None):
        project = ProjectModel(
            code="own",
            name="own",
            description="created by the system",
            author=user,
        )
        project_user = ProjectUserModel(
            project=project,
            user=user,
            current=True,
        )
        self.repository.session.add(project_user)
        if auto_commit:
            await self.repository.session.commit()
