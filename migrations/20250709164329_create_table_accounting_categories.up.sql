CREATE TABLE accounting_categories
(
    id          UUID PRIMARY KEY,
    code        TEXT NOT NULL,
    name        TEXT NOT NULL,
    description TEXT NULL,
    type        TEXT NOT NULL,
    user_id     UUID NOT NULL REFERENCES accounting_users (id),
    UNIQUE(user_id, code)
);