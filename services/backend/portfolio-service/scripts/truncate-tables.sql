-- Fast dev reset without dropping DB — preferred for frequent resets
TRUNCATE assets, transactions, price_history RESTART IDENTITY CASCADE;
