#![allow(unused)] // early dev

use std::sync::Arc;

use common::rabbitmq::{self, RabbitMQ};


pub struct AppState {
    pub rabbitmq: Arc<RabbitMQ>,
}


#[derive(thiserror::Error, Debug)]
enum Error {
    /*
    #[error("Missing environment variable {name}")]
    MissingVar {
        name: &'static str,
        #[source]
        source: std::env::VarError,
    },
     */
}
