use std::sync::Arc;

use lapin::{Channel, Connection, ConnectionProperties};
use tokio::sync::RwLock;
use tracing::info;


pub struct RabbitMQ {
    connection: Arc<RwLock<Option<Connection>>>,
    channel: Arc<RwLock<Option<Channel>>>,
}


impl RabbitMQ {
    pub fn new() -> Self {
        Self {
            connection: Arc::new(RwLock::new(None)),
            channel: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn connect(&self, url: &str) -> Result<(), lapin::Error> {
        let connection = Connection::connect(url, ConnectionProperties::default()).await?;
        info!("RabbitMQ connection established.");

        let channel = connection.create_channel().await?;
        info!("RabbitMQ channel created.");

        //channel.exchange_declare(exchange, kind, options, arguments);

        Ok(())
    }
}


impl Default for RabbitMQ {
    fn default() -> Self {
        Self::new()
    }
}
