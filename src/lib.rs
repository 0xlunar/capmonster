pub mod builders;
mod handlers;
mod types;

use anyhow::format_err;
use crate::builders::Builder;
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

    pub async fn create_task<S: Builder<T>, T: Serialize>(&self, task: S) -> anyhow::Result<T> {
        let task = task.build()?;
        let resp = self
            .client
            .post("https://api.capmonster.cloud/createTask")
            .json(&task)
            .send()
            .await?;

        if resp.status().is_server_error() || resp.status().is_client_error() {
            let status = resp.status();
            let text =
            return Err(format_err!("Error sending request: {} - {}", ));
        }

        Ok()
    }
}

mod test {
    use crate::builders::RecaptchaV2Builder;
    use crate::CapMonster;

    async fn main() {
        let mon = CapMonster::new("");

        let builder = RecaptchaV2Builder::new().build().unwrap();

        let task = mon.create_task(builder).await.unwrap();
    }
}
