# Переменные
DOCKER_USERNAME ?= alexey195
IMAGE_NAME ?= accountingapi
VERSION ?= latest
PLATFORMS = linux/amd64,linux/arm64

# Полное имя образа
FULL_IMAGE_NAME = $(DOCKER_USERNAME)/$(IMAGE_NAME):$(VERSION)

.PHONY: help build push build-push login setup-buildx clean

# Помощь
help:
	@echo "Доступные команды:"
	@echo "  setup-buildx  - Настройка buildx для multi-platform сборки"
	@echo "  build        - Сборка Docker образа для multi-platform"
	@echo "  push         - Загрузка образа в Docker Hub"
	@echo "  build-push   - Сборка и загрузка образа"
	@echo "  login        - Авторизация в Docker Hub"
	@echo "  clean        - Очистка buildx builder"
	@echo ""
	@echo "Переменные окружения:"
	@echo "  DOCKER_USERNAME - имя пользователя Docker Hub (по умолчанию: your-username)"
	@echo "  IMAGE_NAME      - имя образа (по умолчанию: your-app-name)"
	@echo "  VERSION         - версия образа (по умолчанию: latest)"

# Настройка buildx для multi-platform сборки
setup-buildx:
	@echo "Настройка Docker buildx..."
	docker buildx create --name multiarch --driver docker-container --use || true
	docker buildx inspect --bootstrap

# Авторизация в Docker Hub
login:
	@echo "Авторизация в Docker Hub..."
	docker login

# Сборка образа для multiple платформ
build: setup-buildx
	@echo "Сборка образа $(FULL_IMAGE_NAME) для платформ: $(PLATFORMS)"
	docker buildx build \
		--platform $(PLATFORMS) \
		-t $(FULL_IMAGE_NAME) \
		--load \
		.

# Сборка и загрузка образа
build-push: setup-buildx
	@echo "Сборка и загрузка образа $(FULL_IMAGE_NAME) для платформ: $(PLATFORMS)"
	docker buildx build \
		--platform $(PLATFORMS) \
		-t $(FULL_IMAGE_NAME) \
		--push \
		.

# Загрузка уже собранного образа
push:
	@echo "Загрузка образа $(FULL_IMAGE_NAME)..."
	docker push $(FULL_IMAGE_NAME)

# Локальная сборка для текущей платформы (быстрее для разработки)
build-local:
	@echo "Локальная сборка образа $(FULL_IMAGE_NAME)..."
	docker build -t $(FULL_IMAGE_NAME) .

# Запуск контейнера локально
run:
	@echo "Запуск контейнера $(FULL_IMAGE_NAME)..."
	docker run -d -p 8888:8888 --name $(IMAGE_NAME)-container $(FULL_IMAGE_NAME)

# Остановка и удаление контейнера
stop:
	@echo "Остановка контейнера..."
	docker stop $(IMAGE_NAME)-container || true
	docker rm $(IMAGE_NAME)-container || true

# Очистка buildx builder
clean:
	@echo "Очистка buildx builder..."
	docker buildx rm multiarch || true

# Показать информацию о сборке
info:
	@echo "Docker Username: $(DOCKER_USERNAME)"
	@echo "Image Name: $(IMAGE_NAME)"
	@echo "Version: $(VERSION)"
	@echo "Full Image Name: $(FULL_IMAGE_NAME)"
	@echo "Platforms: $(PLATFORMS)"

# Проверка окружения
check:
	@echo "Проверка Docker..."
	docker --version
	@echo "Проверка buildx..."
	docker buildx version
	@echo "Список builders:"
	docker buildx ls

# Генерация Root CA и localhost сертификата, подписанного им
gen-cert:
	@echo "Генерация Root CA..."
	mkdir -p certs
	openssl genrsa -out certs/rootCA.key 2048
	openssl req -x509 -new -nodes -key certs/rootCA.key -sha256 -days 1024 \
		-out certs/rootCA.crt -subj "/CN=MyLocalRootCA"

	@echo "Генерация ключа для localhost..."
	openssl genrsa -out certs/localhost.key 2048

	@echo "Создание CSR..."
	openssl req -new -key certs/localhost.key -out certs/localhost.csr -subj "/CN=localhost"

	@echo "Подпись CSR с помощью Root CA..."
	openssl x509 -req -in certs/localhost.csr -CA certs/rootCA.crt -CAkey certs/rootCA.key \
		-CAcreateserial -out certs/localhost.crt -days 365 -sha256

	@echo "Сертификаты сгенерированы:"
	@echo "  certs/rootCA.crt  (импортируй в доверенные в системе/браузере)"
	@echo "  certs/rootCA.key  (держи в секрете)"
	@echo "  certs/localhost.crt"
	@echo "  certs/localhost.key"

# Приведение dev-сертификатов к универсальным именам
link-dev-certs:
	@echo "Подготовка dev сертификатов под универсальные имена..."
	mkdir -p certs
	cp certs/localhost.crt certs/certificate.crt
	cp certs/localhost.key certs/private.key
	cp certs/rootCA.crt certs/root.crt

# Запуск проекта (с пересборкой образа)
up:
	@echo "Запуск docker-compose со сборкой..."
	docker-compose up --build -d

# Остановка проекта
down:
	@echo "Остановка docker-compose..."
	docker-compose down

# Просмотр логов
logs:
	@echo "Логи docker-compose..."
	docker-compose logs -f

# Перезапуск проекта
restart: down up