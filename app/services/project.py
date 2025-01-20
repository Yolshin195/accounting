from app.models import ProjectUserModel, UserModel
from app.reposiotories.project import ProjectUserRepository
from sqlalchemy.orm import selectinload


class GetUserProjectService:
    def __init__(
            self,
            repo: ProjectUserRepository,
            user: UserModel,
    ):
        self.repo = repo
        self.user = user

    async def execute(self) -> ProjectUserModel:
        user_project = await self.repo.get_one(
            ProjectUserModel.current == True,
            ProjectUserModel.user == self.user,
            load=selectinload(ProjectUserModel.project)
        )
        return user_project
