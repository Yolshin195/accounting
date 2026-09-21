# ============================================================
# Prod Makefile: accounting
# Положить на сервер рядом с compose.yaml, .env и .env-docker
#
# В .env, помимо POSTGRES_*, должны быть:
#   DOMAIN=example.com
#   EMAIL=you@example.com
# ============================================================

# Папка, где лежит этот Makefile (нужна для push, работает из любого места)
HERE := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

# --- Деплой на сервер (make push) ---
# Использование: make push root@1.2.3.4
# Либо один раз сохранить адрес (тогда просто `make push`), файл в git не коммитить:
#   echo "SERVER=root@1.2.3.4" > prod/.server
-include $(HERE).server
SERVER      ?=
SSH_PORT    ?= 22
REMOTE_DIR  ?= /opt/accounting
# Строго фиксированный список файлов из папки prod. Нужен ещё файл: добавь сюда.
DEPLOY_FILES := Makefile compose.prod.yaml .env-example

# `make push root@1.2.3.4`: второе слово после push считаем адресом сервера
ifeq ($(firstword $(MAKECMDGOALS)),push)
  PUSH_ARG := $(word 2,$(MAKECMDGOALS))
  ifneq ($(PUSH_ARG),)
    SERVER := $(PUSH_ARG)
    $(eval $(PUSH_ARG):;@:)
  endif
endif

COMPOSE       ?= docker compose -f compose.prod.yaml
# Имена пользователя и БД Postgres для make env
PG_USER ?= accounting
PG_DB   ?= accounting_db
CERTBOT_IMAGE ?= certbot/certbot:latest
BACKUP_DIR    ?= backups
BACKUP_KEEP_DAYS ?= 14
RENEW_DAYS    ?= 30
# Как часто cron проверяет новые образы (формат cron)
AUTO_UPDATE_SCHEDULE ?= */15 * * * *
# root или sudo — нужен, чтобы поставить пакет cron, если его нет
SUDO := $(shell [ "$$(id -u)" = "0" ] && echo "" || command -v sudo)

# DOMAIN и EMAIL берутся из .env (или: make cert DOMAIN=... EMAIL=...)
DOMAIN ?= $(shell grep -E '^DOMAIN=' .env 2>/dev/null | cut -d= -f2-)
EMAIL  ?= $(shell grep -E '^EMAIL=' .env 2>/dev/null | cut -d= -f2-)

.DEFAULT_GOAL := help
.PHONY: help check check-cert login deploy update up down restart ps logs \
        cert renew-cert ensure-cron install-cron backup \
        auto-update install-auto-update uninstall-auto-update \
        push check-server env

help: ## Список команд
	@grep -hE '^[a-zA-Z_-]+:.*?## ' $(firstword $(MAKEFILE_LIST)) | \
		awk 'BEGIN{FS=":.*?## "}{printf "  %-14s %s\n", $$1, $$2}'

check:
	@command -v docker >/dev/null || { echo "docker не установлен"; exit 1; }
	@test -f .env || { echo "Нет .env. Создай: make env DOMAIN=твой-домен EMAIL=твоя@почта"; exit 1; }

check-cert:
	@test -n "$(DOMAIN)" || { echo "Не задан DOMAIN (в .env или make DOMAIN=...)"; exit 1; }
	@test -n "$(EMAIL)" || { echo "Не задан EMAIL (в .env или make EMAIL=...)"; exit 1; }

env: ## Сгенерировать .env со случайными паролями (make env DOMAIN=... EMAIL=...)
	@command -v openssl >/dev/null || { echo "нужен openssl"; exit 1; }
	@if [ -f .env ] && [ "$(FORCE)" != "1" ]; then \
		echo ".env уже есть, не перезаписываю (новый пароль не подойдёт к существующей БД)."; \
		echo "Перезаписать (старый сохранится в .env.bak): make env FORCE=1"; \
		exit 1; \
	fi
	@domain="$(DOMAIN)"; email="$(EMAIL)"; \
	if [ -z "$$domain" ]; then printf "Домен (example.com): "; read domain; fi; \
	if [ -z "$$email" ]; then printf "Email для Let's Encrypt: "; read email; fi; \
	if [ -z "$$domain" ] || [ -z "$$email" ]; then echo "DOMAIN и EMAIL обязательны"; exit 1; fi; \
	umask 077; \
	if [ -f .env ]; then cp -p .env .env.bak; fi; \
	pass="$$(openssl rand -hex 24)"; jwt="$$(openssl rand -hex 32)"; \
	printf '%s\n' \
		"# Сгенерировано командой make env ($$(date +%F))" \
		"DOMAIN=$$domain" \
		"EMAIL=$$email" \
		"" \
		"POSTGRES_USER=$(PG_USER)" \
		"POSTGRES_PASSWORD=$$pass" \
		"POSTGRES_DB=$(PG_DB)" \
		"" \
		"DATABASE_URL=postgresql://$(PG_USER):$$pass@db:5432/$(PG_DB)" \
		"RUST_LOG=info" \
		"JWT_SECRET=$$jwt" \
		> .env; \
	echo ".env создан (права 600): домен $$domain, пароли сгенерированы"; \
	if docker volume ls -q 2>/dev/null | grep -q 'accounting_postgres_data'; then \
		echo "ВНИМАНИЕ: том БД уже существует. Новый POSTGRES_PASSWORD к нему не применится,"; \
		echo "смени пароль в БД вручную (см. README, шаг 4)."; \
	fi

check-server:
	@test -n "$(SERVER)" || { echo "Не задан сервер. Использование: make push user@host"; exit 1; }

push: check-server ## Скопировать на сервер файлы из DEPLOY_FILES (только prod)
	@for f in $(DEPLOY_FILES); do \
		test -f "$(HERE)$$f" || { echo "Нет файла: $(HERE)$$f"; exit 1; }; \
	done
	ssh -p $(SSH_PORT) $(SERVER) "mkdir -p $(REMOTE_DIR)"
	scp -P $(SSH_PORT) $(addprefix $(HERE),$(DEPLOY_FILES)) $(SERVER):$(REMOTE_DIR)/
	@echo "Скопировано в $(SERVER):$(REMOTE_DIR): $(DEPLOY_FILES)"

login: ## Вход в ghcr.io (нужен PAT с правом read:packages, если образы приватные)
	docker login ghcr.io

deploy: check check-cert ## Первый запуск: сертификат (если нет) + pull + up
	@test -f certs/certificate.crt || $(MAKE) cert
	@$(MAKE) update

update: check ## Обновить образы и перезапустить изменившиеся контейнеры
	$(COMPOSE) pull
	$(COMPOSE) up -d --remove-orphans
	docker image prune -f
	$(COMPOSE) ps

up: check ## Запустить стек
	$(COMPOSE) up -d

down: check ## Остановить стек (данные БД сохраняются)
	$(COMPOSE) down

restart: check ## Перезапустить стек
	$(COMPOSE) restart

ps: check ## Статус контейнеров
	$(COMPOSE) ps

logs: check ## Логи (make logs S=api для одного сервиса)
	$(COMPOSE) logs -f --tail=100 $(S)

cert: check check-cert ## Выпустить/перевыпустить сертификат Let's Encrypt для DOMAIN
	@mkdir -p certs letsencrypt
	@echo "Освобождаю порт 80 (client на время остановится)..."
	@was="$$($(COMPOSE) ps -q --status running client)"; \
	if [ -n "$$was" ]; then $(COMPOSE) stop client; fi; \
	rc=0; \
	docker run --rm -p 80:80 \
		-v "$(CURDIR)/letsencrypt:/etc/letsencrypt" \
		$(CERTBOT_IMAGE) certonly --standalone \
		-d $(DOMAIN) --email $(EMAIL) \
		--agree-tos --no-eff-email --non-interactive --keep-until-expiring \
	&& docker run --rm --entrypoint sh \
		-v "$(CURDIR)/letsencrypt:/etc/letsencrypt:ro" \
		-v "$(CURDIR)/certs:/out" \
		$(CERTBOT_IMAGE) -c '\
			L=/etc/letsencrypt/live/$(DOMAIN) && \
			cp -L $$L/fullchain.pem /out/certificate.crt && \
			cp -L $$L/privkey.pem   /out/private.key && \
			cp -L $$L/chain.pem     /out/root.crt && \
			chmod 644 /out/certificate.crt /out/root.crt && \
			chmod 600 /out/private.key' \
	|| rc=$$?; \
	if [ -n "$$was" ]; then $(COMPOSE) up -d client; fi; \
	if [ $$rc -eq 0 ]; then echo "Сертификаты в ./certs обновлены"; else echo "Ошибка выпуска сертификата"; fi; \
	exit $$rc

renew-cert: check check-cert ## Обновить сертификат, только если осталось < RENEW_DAYS дней
	@if [ -f certs/certificate.crt ] && \
	   openssl x509 -checkend $$(( $(RENEW_DAYS) * 86400 )) -noout -in certs/certificate.crt >/dev/null; then \
		echo "Сертификат действителен ещё > $(RENEW_DAYS) дней, обновление не нужно"; \
	else \
		$(MAKE) cert; \
	fi

ensure-cron: ## Поставить и включить cron, если его ещё нет (используется install-cron/install-auto-update)
	@if ! command -v crontab >/dev/null 2>&1; then \
		echo "crontab не найден, ставлю пакет cron..."; \
		if command -v apt-get >/dev/null 2>&1; then \
			$(SUDO) apt-get update -qq && $(SUDO) apt-get install -y cron; \
		elif command -v yum >/dev/null 2>&1; then \
			$(SUDO) yum install -y cronie; \
		elif command -v apk >/dev/null 2>&1; then \
			$(SUDO) apk add --no-cache cronie; \
		else \
			echo "Не знаю, как поставить cron на этой системе. Поставь вручную (пакет cron/cronie) и повтори."; exit 1; \
		fi; \
	fi
	@if command -v systemctl >/dev/null 2>&1; then \
		$(SUDO) systemctl enable --now cron 2>/dev/null || $(SUDO) systemctl enable --now crond 2>/dev/null || true; \
	elif command -v service >/dev/null 2>&1; then \
		$(SUDO) service cron start 2>/dev/null || $(SUDO) service crond start 2>/dev/null || true; \
	fi
	@command -v crontab >/dev/null 2>&1 || { echo "crontab всё ещё недоступен"; exit 1; }

install-cron: ensure-cron ## Автопродление сертификата (каждый понедельник в 04:00); сама ставит cron, если его нет
	@touch $(CURDIR)/renew.log
	@( crontab -l 2>/dev/null | grep -v 'accounting-cert-renew'; \
	   echo "0 4 * * 1 cd $(CURDIR) && flock $(CURDIR)/.lock $(MAKE) renew-cert >> $(CURDIR)/renew.log 2>&1 # accounting-cert-renew" ) | crontab -
	@echo "Cron установлен:"
	@crontab -l | grep accounting-cert-renew

backup: check ## Дамп БД в ./backups (хранится BACKUP_KEEP_DAYS дней)
	@mkdir -p $(BACKUP_DIR)
	@f="$(BACKUP_DIR)/acc_$$(date +%F_%H%M).sql"; \
	$(COMPOSE) exec -T db sh -c 'pg_dump -U "$$POSTGRES_USER" "$$POSTGRES_DB"' > "$$f" \
		&& gzip "$$f" && echo "Бэкап: $$f.gz" \
		|| { rm -f "$$f"; echo "Ошибка бэкапа"; exit 1; }
	@find $(BACKUP_DIR) -name 'acc_*.sql.gz' -mtime +$(BACKUP_KEEP_DAYS) -delete

auto-update: check ## Проверить новые образы и обновить, если есть (запускается из cron)
	@ids() { for i in $$($(COMPOSE) config --images); do docker image inspect -f '{{.Id}}' "$$i" 2>/dev/null; done | sort | tr '\n' ' '; }; \
	before="$$(ids)"; \
	$(COMPOSE) pull -q || { echo "[$$(date '+%F %T')] pull не удался"; exit 1; }; \
	after="$$(ids)"; \
	if [ "$$before" = "$$after" ]; then \
		if [ -t 1 ]; then echo "Обновлений нет"; fi; \
		exit 0; \
	fi; \
	echo "[$$(date '+%F %T')] найдены новые образы, делаю бэкап и обновляю"; \
	$(MAKE) --no-print-directory backup \
	&& $(COMPOSE) up -d --remove-orphans \
	&& docker image prune -f >/dev/null \
	&& echo "[$$(date '+%F %T')] обновлено" \
	|| { echo "[$$(date '+%F %T')] ОШИБКА обновления"; exit 1; }

install-auto-update: check ensure-cron ## Включить автообновление контейнеров (cron, по умолчанию каждые 15 минут); сама ставит cron и проверяет, что всё работает
	@touch $(CURDIR)/.lock $(CURDIR)/auto-update.log
	@( crontab -l 2>/dev/null | grep -v 'accounting-auto-update'; \
	   echo "$(AUTO_UPDATE_SCHEDULE) cd $(CURDIR) && flock -n $(CURDIR)/.lock $(MAKE) auto-update >> $(CURDIR)/auto-update.log 2>&1 # accounting-auto-update" ) | crontab -
	@echo "Автообновление включено: $(AUTO_UPDATE_SCHEDULE)"
	@crontab -l | grep accounting-auto-update
	@echo "Проверяю, что автообновление действительно работает..."
	@$(MAKE) --no-print-directory auto-update
	@echo "Готово: автообновление настроено и проверено."

uninstall-auto-update: ## Выключить автообновление
	@crontab -l 2>/dev/null | grep -v 'accounting-auto-update' | crontab -
	@echo "Автообновление выключено"