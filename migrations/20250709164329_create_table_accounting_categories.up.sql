CREATE TABLE accounting_categories
(
    id          UUID PRIMARY KEY,
    code        TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL UNIQUE,
    description TEXT NULL,
    type        TEXT NOT NULL,
    user_id     UUID NOT NULL REFERENCES accounting_users (id)
);