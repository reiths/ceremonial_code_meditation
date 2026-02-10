CREATE TYPE transaction_type AS ENUM ('buy', 'sell');

CREATE TABLE assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    name TEXT NOT NULL,
    wkn TEXT UNIQUE,
    isin TEXT UNIQUE,
    ticker TEXT,

    asset_type TEXT NOT NULL DEFAULT 'stock',

    sector TEXT,
    industry TEXT,
    country TEXT,
    currency TEXT,
    exchange TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,

    type transaction_type NOT NULL,

    quantity NUMERIC(18,8) NOT NULL,
    price NUMERIC(18,8) NOT NULL,

    fees NUMERIC(18,8) DEFAULT 0,
    taxes NUMERIC(18,8) DEFAULT 0,

    executed_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE price_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,

    price NUMERIC(18,8) NOT NULL,
    source TEXT,

    recorded_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_transactions_asset_id ON transactions(asset_id);
CREATE INDEX idx_price_history_asset_id ON price_history(asset_id);
