# Accounting API


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