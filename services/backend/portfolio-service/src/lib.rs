mod error;
mod model;

use axum::Router;
use sqlx::PgPool;


#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}


pub fn router(state: AppState) -> Router {
    Router::new()
        //.route("/api/assets", )
        .with_state(state)
}
