#![allow(unused)] // early dev

use std::sync::Arc;

use axum::extract::State;
use client_service::AppState;
use common::rabbitmq::RabbitMQ;
use eyre::Context;
use mongodb::{Client, options::ClientOptions};
use tracing::{info, warn};


#[tokio::main]
async fn main() -> eyre::Result<()> {
    #[cfg(feature = "dev-env")]
    {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("Missing CARGO_MANIFEST_DIR environment variable");

        dotenvy::from_path(format!("{manifest_dir}/.env.dev"))
            .wrap_err_with(|| format!("Failed to load .env file at {}", manifest_dir))?;
    }

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .json()
        .init();

    let mongodb_uri =
        std::env::var("MONGODB_URI").expect("Missing MONGODB_URI environment variable");

    let client_options = ClientOptions::parse(&mongodb_uri).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("client_db");
    info!("{:<12} - MongoDB connection established.", "DB");

    let rabbitmq_url = std::env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://admin:password@rabbitmq:5672".to_string());
    let rabbitmq = Arc::new(RabbitMQ::new());
    if let Err(e) = rabbitmq.connect(&rabbitmq_url).await {
        warn!("Failed to connect to RabbitMQ: {:?}", e);
    } else {
        info!("{:<12} - RabbitMQ connection established.", "RABBITMQ");
    }

    let app_state = Arc::new(AppState { rabbitmq });

    Ok(())
}
