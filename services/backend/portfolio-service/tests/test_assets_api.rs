//! To create the test database use
//! docker exec -i portfolio-db psql -U portfolio_user -d your_test_db < scripts/recreate-test-db.sql

//#![expect(unused)]

use sqlx::PgPool;


#[sqlx::test]
async fn test_create_asset(pool: PgPool) -> sqlx::Result<()> {
    let mut conn = pool.acquire().await?;

    let assets = sqlx::query("SELECT * FROM assets").fetch_all(&mut *conn).await?;

    dbg!(assets);

    Ok(())
}
