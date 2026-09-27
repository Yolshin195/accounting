CREATE TABLE accounting_accounts
(
    id              UUID PRIMARY KEY,
    user_id         UUID    NOT NULL REFERENCES accounting_users (id),
    code            TEXT    NOT NULL,
    name            TEXT    NOT NULL,
    description     TEXT    NULL,
    sort_order      INTEGER NOT NULL DEFAULT 0,
    is_archived     BOOLEAN NOT NULL DEFAULT FALSE,
    currency_id     UUID    NOT NULL REFERENCES accounting_currencies (id),
    account_type    TEXT    NOT NULL,
    initial_balance DECIMAL NOT NULL DEFAULT 0,
    UNIQUE (user_id, code)
);
