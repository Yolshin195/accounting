CREATE TABLE accounting_transactions
(
    id          UUID PRIMARY KEY,
    amount      DECIMAL NOT NULL,
    description TEXT NULL,
    created_at  TIMESTAMP NOT NULL,
    type        TEXT NOT NULL,
    user_id     UUID NOT NULL REFERENCES accounting_users (id),
    category_id UUID NOT NULL REFERENCES accounting_categories (id)
);
