use eyre::Context;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::{debug, info};


#[tokio::main]
async fn main() -> eyre::Result<()> {
    #[cfg(feature = "dev-env")]
    {
        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("Missing CARGO_MANIFEST_DIR environment variable");

        dotenvy::from_path(format!("{manifest_dir}/.env.dev"))
            .wrap_err_with(|| format!("Failed to load .env file at {}", manifest_dir))?;
    }

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug")),
        )
        .json()
        .init();

    let postgres_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");

    let pool = PgPoolOptions::new().max_connections(5).connect(&postgres_url).await?;

    debug!("Connected to Postgres");

    let routes_all = axum::Router::new()
        .route("/health", axum::routing::get(|| async { "ok" }))
        .layer(axum::Extension(pool));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());

    axum::serve(listener, routes_all.into_make_service()).await.unwrap();

    Ok(())
}
