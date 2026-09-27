CREATE TABLE accounting_currencies
(
    id          UUID PRIMARY KEY,
    user_id     UUID    NOT NULL REFERENCES accounting_users (id),
    code        TEXT    NOT NULL,
    name        TEXT    NOT NULL,
    description TEXT    NULL,
    sort_order  INTEGER NOT NULL DEFAULT 0,
    is_archived BOOLEAN NOT NULL DEFAULT FALSE,
    symbol      TEXT    NULL,
    decimals    INTEGER NOT NULL DEFAULT 2,
    is_base     BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (user_id, code)
);

-- У пользователя может быть только одна основная валюта
CREATE UNIQUE INDEX accounting_currencies_one_base_per_user
    ON accounting_currencies (user_id)
    WHERE is_base AND NOT is_archived;
