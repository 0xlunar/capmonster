pub mod builders;
mod handlers;
mod types;

#[cfg(feature = "reqwest")]
use reqwest;
#[cfg(all(feature = "rquest", not(feature = "reqwest")))]
use rquest as reqwest;
use serde::Serialize;

pub struct CapMonster {
    api_key: String,
    client: reqwest::Client,
}

impl CapMonster {
    pub fn new(api_key: &str) -> Self {
        let client = reqwest::Client::builder().build().unwrap();

        Self {
            api_key: api_key.to_string(),
            client,
        }
    }

    pub async fn create_task<S: Serialize>(task: S) -> anyhow::Result<()> {
        Ok(())
    }
}
