use std::sync::Arc;

use lapin::{
    BasicProperties,
    Channel,
    Connection,
    ConnectionProperties,
    ExchangeKind,
    options::{BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
};
use tokio::sync::RwLock;
use tracing::{error, info, trace};


const EVENTS_EXCHANGE: &str = "events";


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

        channel
            .exchange_declare(
                EVENTS_EXCHANGE,
                ExchangeKind::Topic,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        info!("Exchange '{EVENTS_EXCHANGE}' declared.");

        *self.connection.write().await = Some(connection);
        *self.channel.write().await = Some(channel);

        Ok(())
    }

    pub async fn declare_queue(
        &self,
        queue_name: &str,
        routing_key: &str,
    ) -> Result<(), lapin::Error> {
        let channel_guard = self.channel.read().await;
        let channel = channel_guard.as_ref().ok_or_else(|| {
            error!("Channel not initialized!");
            lapin::ErrorKind::InvalidChannel(0)
        })?;

        channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        channel
            .queue_bind(
                queue_name,
                EVENTS_EXCHANGE,
                routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!("Queue '{queue_name}' declared with routing key '{routing_key}'");

        Ok(())
    }

    pub async fn publish(&self, routing_key: &str, message: &[u8]) -> Result<(), lapin::Error> {
        let channel_guard = self.channel.read().await;
        let channel = channel_guard.as_ref().ok_or_else(|| {
            error!("Channel not initialized!");
            lapin::ErrorKind::InvalidChannel(0)
        })?;

        channel
            .basic_publish(
                EVENTS_EXCHANGE,
                routing_key,
                BasicPublishOptions::default(),
                message,
                BasicProperties::default()
                    .with_content_type("application/json".into()) // tell the consumers this is a json event
                    .with_delivery_mode(2) // persistant
                    .with_timestamp(chrono::Utc::now().timestamp() as u64),
            )
            .await? // send the message to RabbitMQ
            .await?; // wait for broker confirmation

        trace!("Message published to '{routing_key}'");
        Ok(())
    }

    pub async fn get_channel(&self) -> Option<Channel> {
        self.channel.read().await.clone()
    }
}


impl Default for RabbitMQ {
    fn default() -> Self {
        Self::new()
    }
}
