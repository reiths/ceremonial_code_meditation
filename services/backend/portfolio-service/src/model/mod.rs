use sqlx::PgPool;

pub mod asset;
mod base;
mod error;
mod store;


/// Kinda like the application state
#[derive(Clone)]
pub struct ModelManager {
    db: PgPool,
}


impl ModelManager {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let db = store::database_pool().await?;
        Ok(ModelManager { db })
    }

    /// The database pool is only for the model layer
    pub(in crate::model) fn db(&self) -> &PgPool {
        &self.db
    }
}
