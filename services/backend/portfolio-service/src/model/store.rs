use sqlx::{PgPool, postgres::PgPoolOptions};


pub async fn database_pool() -> sqlx::Result<PgPool> {
    let database_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
}
