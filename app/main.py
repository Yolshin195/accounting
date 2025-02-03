from litestar import Litestar, get
from litestar.contrib.sqlalchemy.plugins import SQLAlchemyPlugin
from litestar.di import Provide

from app.controllers import router
from app.core import provide_limit_offset_pagination
from app.db import sqlalchemy_config

from litestar.openapi.config import OpenAPIConfig
from litestar.openapi.plugins import SwaggerRenderPlugin

from app.security import litestar_users


@get("/")
async def hello_world() -> dict[str, str]:
    """Handler function that returns a greeting dictionary."""
    return {"hello": "world"}


app = Litestar(
    route_handlers=[router, hello_world],
    plugins=[
        SQLAlchemyPlugin(sqlalchemy_config),
        litestar_users
    ],
    openapi_config=OpenAPIConfig(
        title="Litestar Example",
        description="Example of Litestar with Scalar OpenAPI docs",
        version="0.0.1",
        render_plugins=[SwaggerRenderPlugin()],
    ),
    debug=True,
    dependencies={"limit_offset": Provide(provide_limit_offset_pagination, sync_to_thread=False)},
)
