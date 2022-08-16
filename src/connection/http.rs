use async_trait::async_trait;

use crate::connection::{ChargerConnection, GoEError};
use crate::{GoEStatus, StatusJson};

pub struct DirectHttpChargerConnection {
    host: String,
}

impl DirectHttpChargerConnection {
    pub fn new(host: String) -> Self {
        Self { host }
    }

    fn base_url(&self) -> String {
        format!("http://{}/", self.host)
    }

    async fn get_status_json(&self) -> Result<StatusJson, reqwest::Error> {
        let endpoint = format!("{}status", self.base_url());

        let status = reqwest::get(endpoint).await?.json().await?;

        Ok(status)
    }
}

#[async_trait]
impl ChargerConnection for DirectHttpChargerConnection {
    type ConnError = reqwest::Error;

    async fn set_key(&mut self, key: String, value: String) -> Result<(), Self::ConnError> {
        Ok(())
    }

    async fn latest_status(&self) -> Result<GoEStatus, GoEError> {
        let status_json = self.get_status_json().await?;

        let status = GoEStatus::try_from_status_json(&status_json)?;

        Ok(status)
    }
}
