# Используем официальный образ Rust с полной средой
FROM rust:trixie

ENV SQLX_OFFLINE=true

# Устанавливаем системные зависимости для сборки (sqlx, argon2 и др.)
RUN apt-get update && \
    apt-get install -y libssl-dev pkg-config build-essential && \
    rm -rf /var/lib/apt/lists/*

# Устанавливаем рабочую директорию
WORKDIR /usr/src/accounting

# Копируем все файлы проекта
COPY . .

# Сборка проекта в release
RUN cargo build --release

# Открываем порт
EXPOSE 8888

# Запуск бинарника
CMD ["./target/release/accounting"]
