# Accounting API

## Docker: сборка и деплой (Linux/Debian сервер + MacBook M3)

Образ `ghcr.io/yolshin195/accountingapi` собирается как multi-arch
(`linux/amd64` + `linux/arm64`) и публикуется под тегом `latest` в GitHub
Container Registry, поэтому один и тот же `compose.prod.yaml` работает без
изменений и на удалённом Debian-сервере (amd64), и локально на MacBook M3
(arm64) — Docker сам подтягивает нужный слой образа под архитектуру хоста.

### CI/CD

Сборка и публикация в GitHub Container Registry (ghcr.io) выполняются
автоматически workflow'ом `.github/workflows/docker-publish.yml` при пуше в
`main` (или вручную через "Run workflow"). Публикация использует
встроенный `GITHUB_TOKEN` — отдельные секреты для реестра не нужны, но
у токена должно быть право `packages: write` (уже выставлено в workflow
через `permissions:`).

Пакет `accountingapi` создаётся приватным по умолчанию. Чтобы
`docker compose pull` на удалённом сервере и на Mac работал без логина,
сделайте пакет публичным: GitHub → профиль/организация → Packages →
`accountingapi` → Package settings → Change visibility → Public. Если
пакет должен оставаться приватным, перед `pull` нужно один раз выполнить
`docker login ghcr.io -u <username>` с personal access token (`read:packages`)
и на сервере, и на Mac.

Для автодеплоя на удалённый сервер нужно добавить в репозиторий:

**Secrets** (Settings → Secrets and variables → Actions → Secrets):
- `DEPLOY_USER`, `DEPLOY_SSH_KEY` — пользователь и приватный SSH-ключ для деплоя на удалённый сервер.

**Variables** (там же, вкладка Variables):
- `DEPLOY_HOST` — адрес удалённого Debian-сервера.
- `DEPLOY_PATH` — путь к проекту на сервере (где лежит `compose.prod.yaml`).
- `DEPLOY_PORT` — SSH-порт (необязательно, по умолчанию 22).

Если `DEPLOY_HOST` не задан, шаг деплоя на сервер пропускается — workflow
только соберёт и опубликует образ.

### Запуск на MacBook M3

CI не может деплоить на локальный Mac, поэтому после публикации образа
достаточно обновить его вручную:

```shell
docker compose -f compose.prod.yaml pull
docker compose -f compose.prod.yaml up -d
```

Docker автоматически скачает `arm64`-вариант образа.

### Ручная сборка (без CI)

Через `Makefile` (использует `docker buildx` и те же платформы):

```shell
make login   # docker login ghcr.io, один раз
make build-push
```

## SqlX migration
create migration
```shell
sqlx migrate add -r migration_name
```

# API Documentation

Система управления личными финансами с возможностью отслеживания доходов и расходов.

## Аутентификация

### Регистрация пользователя
```
POST /users/register
```

**Тело запроса:**
```json
{
  "username": "alex",
  "password": "alex"
}
```

Создает новую учетную запись пользователя.

### Авторизация
```
POST /users/login
```

**Тело запроса:**
```json
{
  "username": "alex",
  "password": "alex"
}
```

Возвращает JWT токен для аутентификации последующих запросов.

## Управление категориями

### Создание категории
```
POST /categories
Authorization: Bearer {token}
```

**Тело запроса:**
```json
{
  "code": "FOOD1",
  "name": "Еда1",
  "type": "INCOME"
}
```

Создает новую категорию транзакций с уникальным кодом и типом (INCOME/EXPENSE).

### Получение списка категорий
```
GET /categories?size=10&page=0
Authorization: Bearer {token}
```

Возвращает пагинированный список всех категорий пользователя.

### Удаление категории
```
DELETE /categories/{code}
Authorization: Bearer {token}
```

Удаляет категорию по её коду.

## Управление транзакциями

### Создание расходной транзакции
```
POST /transactions/expense
Authorization: Bearer {token}
```

**Тело запроса:**
```json
{
  "amount": 100,
  "category": "FOOD1",
  "description": "Test create transaction"
}
```

Создает новую транзакцию расхода с указанной суммой, категорией и описанием.

### Создание доходной транзакции
```
POST /transactions/income
Authorization: Bearer {token}
```

**Тело запроса:**
```json
{
  "amount": 100,
  "category": "FOOD1",
  "description": "Test create transaction",
  "date": "2025-07-01"
}
```

Создает новую транзакцию дохода. Дата может быть указана в прошлом.

### Получение списка транзакций
```
GET /transactions?size=10&page=0
Authorization: Bearer {token}
```

Возвращает пагинированный список всех транзакций пользователя.

### Получение транзакции по ID
```
GET /transactions/{id}
Authorization: Bearer {token}
```

Возвращает детальную информацию о конкретной транзакции.

### Обновление транзакции
```
PUT /transactions/{id}
Authorization: Bearer {token}
```

**Тело запроса:**
```json
{
  "amount": 1535.5,
  "category": "DEBT_REPAYMENT",
  "description": "Друг вернул долг, я за него заплатил в баре тест",
  "date": "2025-08-17T05:50:29.875848Z"
}
```

Полностью обновляет существующую транзакцию.

### Получение месячной сводки
```
GET /transactions/month
Authorization: Bearer {token}
```

Возвращает сводку доходов и расходов за текущий месяц.

### Получение расходов за сегодня
```
GET /transactions/expenses/today
Authorization: Bearer {token}
```

Возвращает все расходные транзакции за текущий день.

## Авторизация

Все защищенные эндпоинты требуют передачи JWT токена в заголовке:
```
Authorization: Bearer {your_jwt_token}
```


Токен получается после успешной авторизации через `/users/login`.


### Архитектура портов и адаптеров

Входящий адаптер [Хендлер]

Бизнес логика [Сервисы]

Исходящий адаптер [Репозиторий]

Если нужно использовать два сервиса, то мы не пытаемся объединить передать один сервис в другой, мы создаем новый сервис агрегат

###

src
    - Адаптеры (Адаптирую данные для бизнес логике сервисов)
        - Входящие (Хендлеры)
            - http
            - grpc
            - telegram bot
        - Исходящие (Репозитории)
            - user
                - postgresql
                - redis
            - category
            - transaction
    - Порты (Содержит только интерфейсы, для адаптеров и сервисов)
        - Порты для Адептеры
        - Порты для сервисов
    - Сервисы (Реализация)
        -
