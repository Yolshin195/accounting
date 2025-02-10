# accounting

## v1.0.0
* [ ] Реализовать справочники
    + [X] Категории
    + [X] Валюты
    + [X] Счета
    + [ ] сделать code уникальным в рамках одного project
    + [ ] настроить запрет на примой прямой commit в мастер

### Запуск проекта
1. Установить UV: [Документация UV](https://docs.astral.sh/uv/getting-started/installation/)
2. Скачать зависимости
```shell
  uv sync --frozen
 ```
3. Запуск проекта
```shell
  litestar run
```

### Запуск через Docker-compose
```shell
docker-compose up
```
Для фоновоого режима:
```shell
docker-compose up -d
```

### Зайти в консоль контейнера
```shell
docker build -t accounting_api . && docker run -it --entrypoint /bin/bash accounting_api
```


