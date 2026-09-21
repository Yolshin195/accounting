# Прод-деплой accounting

Стек: `client` (nginx, 80/443) → `api` → `db` (Postgres). Наружу опубликованы только порты клиента, API и БД доступны лишь внутри docker-сетей.

## Файлы

| Файл | Где | Назначение |
|---|---|---|
| `Makefile` | репозиторий и сервер | все команды деплоя |
| `compose.prod.yaml` | репозиторий и сервер | описание стека |
| `.env-example` | репозиторий и сервер | справочный список ключей |
| `.env` | только сервер | создаётся `make env`: домен, email, `POSTGRES_*`, `DATABASE_URL`, `JWT_SECRET`, `RUST_LOG` |

`.env` один на всё: его читает compose (подстановки `${...}`), из него же `api` получает переменные (`env_file: .env` в compose), а Makefile берёт `DOMAIN` и `EMAIL`. Файл содержит секреты: в git не коммитить, права 600 (`make env` ставит их сам).

## Что нужно заранее

- VDS с Docker и плагином `docker compose`, SSH-доступ по ключу.
- Домен, A-запись которого указывает на IP сервера.
- Открытые порты 22, 80 и 443 (в ufw и в панели хостера).

## Первый деплой

**1. Подготовить сервер** (один раз)
```bash
ufw allow 22/tcp && ufw allow 80/tcp && ufw allow 443/tcp && ufw enable
```

**2. Загрузить файлы** (с локальной машины, из папки `prod`)
```bash
make push root@1.2.3.4
```

**3. Настроить окружение** (на сервере, в `/opt/accounting`)
```bash
make env DOMAIN=твой-домен.ru EMAIL=ты@почта.ru
```
Пароль Postgres и `JWT_SECRET` генерируются случайно (hex, безопасно для URL), `DATABASE_URL` собирается с тем же паролем и хостом `db`. Без `DOMAIN`/`EMAIL` команда спросит их сама. Существующий `.env` она не перезапишет; принудительно: `make env FORCE=1` (старый уйдёт в `.env.bak`). Другое имя пользователя или БД: `make env PG_USER=... PG_DB=...`.

**4. Только если БД уже существовала** (том создан раньше): `POSTGRES_USER` и `POSTGRES_PASSWORD` из нового `.env` на существующий том не влияют. Оставь прежнего пользователя и смени ему пароль, пока БД работает:
```bash
make env PG_USER=postgres          # вместо шага 3
docker compose -f compose.prod.yaml exec db psql -U postgres -c "ALTER USER postgres PASSWORD '$(grep ^POSTGRES_PASSWORD= .env | cut -d= -f2-)';"
```

**5. Войти в реестр** (только если образы в ghcr.io приватные; PAT с правом `read:packages`)
```bash
make login
```

**6. Запустить**
```bash
make deploy          # сертификат Let's Encrypt, pull, up
```

**7. Включить автоматику**
```bash
make install-cron          # автопродление сертификата, пн 04:00
make install-auto-update   # проверка новых образов каждые 15 минут
```

**8. Проверить**
```bash
make ps                          # все сервисы Up
ss -tlnp | grep -E ':(80|443|8888|5432|5445)\b'   # наружу только 80 и 443
curl -I https://ДОМЕН
```

## Обновление

- **Новые образы** подтягиваются автоматически, перед этим делается бэкап БД. Вручную: `make update`.
- **Изменились файлы деплоя** (compose, Makefile): с локальной машины `make push root@1.2.3.4`, затем на сервере `make update`.

## Команды

| Команда | Действие |
|---|---|
| `make env` | сгенерировать `.env` (`DOMAIN=... EMAIL=...`) |
| `make push user@host` | скопировать на сервер файлы из `DEPLOY_FILES` (локально) |
| `make deploy` | первый запуск |
| `make update` | pull образов и перезапуск изменившихся |
| `make auto-update` | то же, но только если есть новые образы (для cron) |
| `make cert` / `make renew-cert` | выпуск / продление сертификата |
| `make backup` | дамп БД в `./backups`, хранятся 14 дней |
| `make logs S=api` | логи сервиса |
| `make up` / `down` / `restart` / `ps` | управление стеком |
| `make help` | полный список |

## Бэкапы

Ежедневный дамп (cron на сервере, `crontab -e`):
```
0 3 * * * cd /opt/accounting && make backup
```
Восстановление:
```bash
gunzip -c backups/acc_ДАТА.sql.gz | docker compose -f compose.prod.yaml exec -T db sh -c 'psql -U "$POSTGRES_USER" "$POSTGRES_DB"'
```
Копии стоит хранить не только на этом же сервере.

## Если что-то не работает

| Симптом | Причина |
|---|---|
| `make cert` падает | A-запись не указывает на сервер, порт 80 закрыт, или ранее упало слишком много попыток (лимиты Let's Encrypt) |
| `client` перезапускается | nginx не находит `certs/certificate.crt`, `private.key`, `root.crt`: проверь `ls certs/` и `make logs S=client` |
| `api` не подключается к БД | пароль в БД не совпадает с `.env` (том создан раньше, см. шаг 4) или `DATABASE_URL` правили руками: хост должен быть `db` |
| Фронт не достучался до API | запросы должны идти через nginx клиента (`/api/...`), порт 8888 наружу закрыт |
| Автообновление ничего не делает | образы закреплены на версии вместо `:latest`; смотри `auto-update.log` |