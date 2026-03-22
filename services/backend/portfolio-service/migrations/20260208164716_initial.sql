CREATE TYPE asset_type AS ENUM ('stock', 'etf', 'fund', 'bond', 'crypto');

CREATE TYPE transaction_type AS ENUM ('buy', 'sell');

CREATE TYPE currency AS ENUM ('eur', 'usd');

CREATE TABLE assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    name TEXT NOT NULL,
    wkn TEXT UNIQUE NOT NULL,
    isin TEXT UNIQUE,
    ticker TEXT,

    asset_type asset_type NOT NULL,

    sector TEXT,
    industry TEXT,
    country TEXT,
    currency currency NOT NULL,
    exchange TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE RESTRICT,

    transaction_type transaction_type NOT NULL,

    quantity NUMERIC(18,8) NOT NULL CHECK (quantity > 0),
    price NUMERIC(18,8) NOT NULL CHECK (price >= 0),

    fees NUMERIC(18,8) DEFAULT 0 CHECK (fees >= 0),
    taxes NUMERIC(18,8) DEFAULT 0 CHECK (taxes >= 0),

    executed_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_transactions_asset_id ON transactions(asset_id);
