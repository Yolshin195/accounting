################################################################
# Стадия 1: подготовка "рецепта" зависимостей (cargo-chef)
# Позволяет кэшировать компиляцию зависимостей отдельно от
# исходного кода — при изменении только src/ зависимости
# пересобираться не будут.
################################################################
FROM rust:1-slim-trixie AS chef
WORKDIR /app
RUN cargo install cargo-chef --locked

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

################################################################
# Стадия 2: сборка зависимостей и приложения
################################################################
FROM chef AS builder

# build-essential нужен только для нескольких крейтов с build.rs
# (например, ring), собирающих небольшой C-код. Дополнительные
# системные библиотеки (libssl-dev и т.п.) не требуются: sqlx
# используется с runtime-tokio-rustls, без OpenSSL.
RUN apt-get update && \
    apt-get install -y --no-install-recommends build-essential && \
    rm -rf /var/lib/apt/lists/*

ENV SQLX_OFFLINE=true

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin accounting

################################################################
# Стадия 3: минимальный рантайм-образ
################################################################
FROM debian:trixie-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    useradd --system --no-create-home --uid 10001 appuser

WORKDIR /app
COPY --from=builder /app/target/release/accounting ./accounting

USER appuser
EXPOSE 8888

ENTRYPOINT ["./accounting"]
