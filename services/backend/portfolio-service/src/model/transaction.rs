use rust_decimal::Decimal;
use sqlx::types::{
    Uuid,
    chrono::{DateTime, Utc},
};


#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
pub enum TransactionType {
    Buy,
    Sell,
}


#[derive(Debug, sqlx::FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub transaction_type: TransactionType,
    pub quantity: Decimal,
    pub price: Decimal,
    pub fees: Decimal,
    pub taxes: Decimal,
    pub executed_at: DateTime<Utc>,
}
