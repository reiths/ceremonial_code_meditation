#![expect(unused)] // early dev

use axum::Router;

pub mod context;
mod error;
mod model;


#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}


pub fn router(state: AppState) -> Router {
    Router::new()
        //.route("/api/assets", )
        .with_state(state)
}
