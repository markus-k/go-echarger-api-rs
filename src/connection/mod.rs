use async_trait::async_trait;

use crate::{GoEError, GoEStatus};

pub mod http;
pub mod mqtt;

#[async_trait]
pub trait ChargerConnection {
    async fn set_key(&mut self, key: &str, value: &str) -> Result<(), GoEError>;
    async fn latest_status(&self) -> Result<GoEStatus, GoEError>;
}
