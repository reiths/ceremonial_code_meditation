use sqlx::types::{
    Uuid,
    chrono::{DateTime, Utc},
};


#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "asset_type", rename_all = "lowercase")]
pub enum AssetType {
    Stock,
    Etf,
    Fund,
    Bond,
    Crypto,
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "currency", rename_all = "lowercase")]
pub enum Currency {
    Eur,
    Usd,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Asset {
    pub id: Uuid,
    pub name: String,
    pub wkn: String,
    pub isin: Option<String>,
    pub ticker: Option<String>,
    pub asset_type: AssetType,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub country: Option<String>,
    pub currency: Currency,
    pub exchange: Option<String>,
    //pub created_at: DateTime<Utc>,
    //pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AssetForTransaction {
    pub id: Uuid,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AssetForDelete {
    pub id: Uuid,
}

pub async fn delete_asset(pool: &sqlx::PgPool, cmd: AssetForDelete) -> Result<(), sqlx::Error> {
    /*
    sqlx::query!("DELETE FROM assets WHERE id = $1", cmd.id)
        .execute(pool)
        .await?;
    */
    todo!();
    Ok(())
}


/*
pub async fn fetch_assets(pool: &sqlx::PgPool) -> Result<Vec<Asset>, sqlx::Error> {
    sqlx::query_as::<_, Asset>("SELECT * FROM assets").fetch_all(pool).await
}

pub async fn active_assets(pool: &sqlx::PgPool) -> Result<Vec<(Asset, Decimal)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT a.*, COALESCE(SUM(
            CASE WHEN t.type = 'buy' THEN t.quantity
                 WHEN t.type = 'sell' THEN -t.quantity
            END), 0) AS net_quantity
        FROM assets a
        LEFT JOIN transactions t ON t.asset_id = a.id
        GROUP BY a.id
        HAVING COALESCE(SUM(
            CASE WHEN t.type = 'buy' THEN t.quantity
                 WHEN t.type = 'sell' THEN -t.quantity
            END), 0) > 0
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| {
            let asset = Asset {
                id: r.id,
                name: r.name,
                wkn: r.wkn,
                isin: r.isin,
                ticker: r.ticker,
                asset_type: r.asset_type,
                sector: r.sector,
                industry: r.industry,
                country: r.country,
                currency: r.currency,
                exchange: r.exchange,
            };
            let net_quantity = r.net_quantity.unwrap_or_else(|| Decimal::ZERO);
            (asset, net_quantity)
        })
        .collect())
}
*/


/*
pub async fn insert(pool: &sqlx::PgPool, asset: AssetCreate) -> sqlx::Result<()> {
    sqlx::query(
        r#"INSERT INTO assets (id, amount) VALUES ($1, $2)"#).bind(value)
        asset.id,
        asset.amount,
    )
    .execute(pool)
    .await?;

Ok(())
}
*/
