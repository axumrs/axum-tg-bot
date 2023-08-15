use serde::{Deserialize, Serialize};

use crate::{ChatID, Error, Result};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub token: String,
    pub webhook_url: String,
    pub webhook_domain: String,
    pub chat_id: ChatID,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(Error::from)?
            .try_deserialize()
            .map_err(Error::from)
    }

    pub fn full_webhook_url(&self) -> String {
        format!("{}{}", self.webhook_domain, self.webhook_url)
    }
}
