# Задание на разработку: Методы работы со справочником `Account`

## Цель задачи:
Необходимо реализовать CRUD-методы (создание, получение, обновление, удаление), а также метод получения списка объектов для справочника `Account`, по аналогии с уже существующими реализациями для справочников `Category` и `Currency`. Кроме того, важно учитывать, что справочник `Account` имеет связь с другими справочниками (`Currency`), поэтому потребуется создать собственные DTO, наследованные от базовых DTO (`ReferenceDTO`).

---

## Результат:
1. Создать `Service`, `Controller` и необходимые интеграции для справочника `Account`.
2. Создать соответствующие DTO для `Account`, учитывая их связь с `Currency`.
3. Код должен быть написан в рамках существующего проекта и соответствовать текущей архитектуре.

---

## Описание задач

### 1. Создать репозиторий для `Account`
Аналогично уже существующим репозиториям `CategoryRepository` и `CurrencyRepository`, необходимо реализовать репозиторий `AccountRepository`.
- Репозиторий находится в файле `app.repositories.reference`.
- Используй модель `AccountModel`. Аналогично другим классам репозиториев, классу нужно наследоваться от `repository.SQLAlchemyAsyncRepository`.

Пример:
```python
from app.models import AccountModel
from litestar.plugins.sqlalchemy.repository import SQLAlchemyAsyncRepository

class AccountRepository(SQLAlchemyAsyncRepository[AccountModel]):
    """Account repository."""
    model_type = AccountModel
```

---

### 2. Создать DTO для `Account`
Поскольку справочник `Account` имеет связь с `Currency`, нужно создать специализированные DTO для работы с этим справочником. 

#### Шаги:
1. **Создать `AccountDTO` и `CreateAccountDTO`.**
   - Наследовать их от базовых DTO (`ReferenceDTO`, `CreateReferenceDTO`).
   - Добавить поле `currency_id` (UUID) для ссылки на справочник валюты.

Пример:
```python
from app.schema import ReferenceDTO, CreateReferenceDTO, UpdateReferenceDTO
from uuid import UUID

class AccountDTO(ReferenceDTO):
    currency_id: UUID

class CreateAccountDTO(CreateReferenceDTO):
    currency_id: UUID

class UpdateAccountDTO(UpdateReferenceDTO):
    currency_id: UUID
```

2. **Назначить связь в сервисах.**
   - Убедись, что в процессе обработки данных эти новые DTO корректно передают `currency_id` в базу данных.

---

### 3. Создать сервис для работы с `Account`
- Создай новый сервис `AccountService` в `app.services.reference`.
- Наследуй его от `service.SQLAlchemyAsyncRepositoryService` (как, например, `CategoryService` или `CurrencyService`).
- В сервисе должны быть методы:
  - Получение всех объектов — метод `get_all`.
  - Получение объекта по UUID — метод `get_by_id`.
  - Создание объекта — метод `add`.
  - Обновление объекта — метод `update`.
  - Удаление объекта — метод `delete`.
  
**Используй объект `AccountRepository` (созданный на этапе 1) в качестве `repository_type`. Также по аналогии с другими сервисами добавь использование `user_project`.**

Пример:
```python
from app.models import AccountModel, ProjectUserModel
from app.repositories.reference import AccountRepository
from app.schema import AccountDTO, CreateAccountDTO
from litestar.plugins.sqlalchemy.service import SQLAlchemyAsyncRepositoryService

class AccountService(SQLAlchemyAsyncRepositoryService[AccountModel]):
    """Account service to handle CRUD operations."""
    repository_type = AccountRepository

    def __init__(self, *args, user_project: ProjectUserModel, **kwargs):
        super().__init__(*args, **kwargs)
        self.user_project = user_project

    async def add(self, data: CreateAccountDTO) -> None:
        # Логика, которая связывает account с currency_id
        model = await self.to_model(data.__dict__)
        model.project_id = self.user_project.project_id
        await self.create(model, auto_commit=True)
```

---

### 4. Создать контроллер для работы с `Account`
- Создай файл/класс `AccountController` в `app.controllers.reference`.
- Наследуй контроллер от `Controller`. Путь для контроллера должен быть `/account`.
- Реализуй методы:
  - `GET /` — получение списка счетов с поддержкой фильтров-пагинации (аналогично `category_list` или `currency_list`).
  - `GET /{item_id:uuid}` — получение счета по UUID.
  - `POST /` — создание нового счета.
  - `PUT /{item_id:uuid}` — обновление счета.
  - `DELETE /{item_id:uuid}` — удаление счета.

**Подсказка:** Реализацию методов можно посмотреть в `CategoryController` и `CurrencyController`.

Пример:
```python
from litestar import Controller, get, post, put, delete
from app.services.reference import AccountService
from app.schema import AccountDTO, CreateAccountDTO, UpdateAccountDTO

class AccountController(Controller):
    path = "/account"
    dependencies = {
        "service": Provide(get_account_service)
    }

    @get("/")
    async def account_list(self, limit_offset: filters.LimitOffset, service: AccountService):
        return await service.get_all(limit_offset)

    @get("/{item_id:uuid}")
    async def get_account(self, item_id: UUID, service: AccountService):
        return await service.get_by_id(item_id)

    @post("/")
    async def create_account(self, data: CreateAccountDTO, service: AccountService):
        await service.add(data)

    @put("/{item_id:uuid}")
    async def update_account(self, item_id: UUID, data: UpdateAccountDTO, service: AccountService):
        await service.update(data, item_id=item_id, auto_commit=True)

    @delete("/{item_id:uuid}")
    async def delete_account(self, item_id: UUID, service: AccountService):
        await service.delete(item_id=item_id, auto_commit=True)
```

---

### 5. Обеспечить интеграцию
- Добавь необходимые зависимости для работы контроллера. Пример работы зависимости можно увидеть в `dependencies` внутри `CategoryController` или `CurrencyController`. Она связывается с `AccountService`.
- Убедись, что связи между `Account` и `Currency` корректно работают в методах `add` и `update`.
- Для подключения контроллера добавь его в основное приложение (если этого еще не сделано).

---

### 6. Тестирование
- С помощью Postman или curl проверь работу всех методов контроллера:
    - Создание записи и привязка к `currency_id`.
    - Обновление записи с изменением `currency_id`.
    - Удаление записей.
    - Получение одной записи по UUID.
    - Пагинация списка записей.
- Проверь, что правильные данные сохраняются и обновляются в базе данных.

---

## Где искать подсказки:
1. **Репозитории.** Описание шаблона репозитория смотри в `CategoryRepository` и `CurrencyRepository` (файл `reference.py`, `app.repositories.reference`).
2. **Сервисы.** Аналоги есть в `CategoryService`, `CurrencyService` (`app.services.reference`).
3. **Контроллеры.** Аналогичный функционал можно найти в `CategoryController`, `CurrencyController` (`app.controllers.reference`).
4. **DTO.** См. `ReferenceDTO`, `CreateReferenceDTO`, `UpdateReferenceDTO` для создания собственных DTO в `schema.py`.
5. **Модель `Account`.** Находится в файле `models.py`.

---

## Требования:
1. Код должен быть читаемым, написанным в едином стиле.
2. Все методы контроллера должны быть протестированы на корректность.
3. Поддержка поля `currency_id` обязательна.
4. Рекомендуется ориентироваться на существующий код для остальных справочников, чтобы сохранить единый стиль разработки. 

Удачи! 😊