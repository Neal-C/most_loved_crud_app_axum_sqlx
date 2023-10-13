-- Add migration script here
CREATE TABLE IF NOT EXISTS quote (
    id UUID PRIMARY KEY,
    book VARCHAR(63) NOT NULL,
    quote VARCHAR(255) NOT NULL,
    inserted_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    UNIQUE(book, quote)
)