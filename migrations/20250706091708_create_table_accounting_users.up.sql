CREATE TABLE accounting_users
(
    id            UUID PRIMARY KEY,
    telegram_id   TEXT NULL UNIQUE,
    username      TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);
