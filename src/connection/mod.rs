use async_trait::async_trait;
use thiserror::Error;

use crate::{GoEStatus, GoEStatusError};

pub mod http;
pub mod mqtt;

pub struct GoECharger<C: ChargerConnection> {
    connection: C,
}

impl<C: ChargerConnection> GoECharger<C> {
    pub fn new(connection: C) -> Self {
        Self { connection }
    }
}

#[derive(Debug, Error)]
pub enum GoEError {
    #[error("Connection error")]
    HttpError(#[from] reqwest::Error),
    // #[cfg(feature = "mqtt")]
    // MqttError(#[from] ???),
    #[error("Status parsing error")]
    StatusError(#[from] GoEStatusError),
}

#[async_trait]
pub trait ChargerConnection {
    type ConnError;

    async fn set_key(&mut self, key: String, value: String) -> Result<(), Self::ConnError>;
    async fn latest_status(&self) -> Result<GoEStatus, GoEError>;
}
