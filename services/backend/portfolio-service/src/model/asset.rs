pub struct Asset {}

#[derive(serde::Deserialize)]
pub struct AssetCreate {
    pub symbol: String,
    //pub amount: Decimal,
}

pub struct AssetUpdate {}

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
