use async_trait::async_trait;

use crate::connection::{ChargerConnection, GoEError};
use crate::status::{GoEStatus, StatusJson};

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
    async fn set_key(&mut self, key: &str, value: &str) -> Result<(), GoEError> {
        let endpoint = format!("{}mqtt", self.base_url());
        let _new_status = reqwest::Client::new()
            .request("SET".parse().unwrap(), endpoint)
            .query(&[("payload", format!("{key}={value}"))])
            .send()
            .await?
            .json::<StatusJson>()
            .await?;

        Ok(())
    }

    async fn latest_status(&self) -> Result<GoEStatus, GoEError> {
        let status_json = self.get_status_json().await?;

        let status = GoEStatus::try_from_status_json(&status_json)?;

        Ok(status)
    }
}
