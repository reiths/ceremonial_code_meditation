#![allow(unused)] // early dev

use eyre::Context;


#[tokio::main]
async fn main() -> eyre::Result<()> {
    #[cfg(feature = "dev-env")]
    {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("Missing CARGO_MANIFEST_DIR environment variable");

        dotenvy::from_path(format!("{manifest_dir}/.env"))
            .wrap_err_with(|| format!("Failed to load .env file at {}", manifest_dir))?;
    }

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .json()
        .init();

    /*
    let mongodb_uri =
        std::env::var("MONGODB_URI").expect("Missing MONGODB_URI environment variable");
    */


    Ok(())
}
