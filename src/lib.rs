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
use crate::types::{CapMonsterError, CreateTask, CreateTaskResponse, GetTaskResultPayload, GetTaskResultResponse, TaskCaptchaType, TaskId};

static BASE_URL: &'static str = "https://api.capmonster.cloud";

pub struct CapMonster {
    api_key: String,
    callback_url: Option<String>,
    client: reqwest::Client,
}

impl CapMonster {
    pub fn new(api_key: &str, callback_url: Option<String>) -> Self {
        let client = reqwest::Client::builder().build().unwrap();

        Self {
            api_key: api_key.to_string(),
            callback_url,
            client,
        }
    }

    pub async fn create_task<S: Builder<T>, T: Serialize>(&self, task: S) -> anyhow::Result<CapMonsterTask> {
        let task_type = task.get_type();
        let task = task.build()?;
        let create_task = CreateTask {
            client_key: self.api_key.clone(),
            task,
            callback_url,
        };

        let resp = self
            .client
            .post("https://api.capmonster.cloud/createTask")
            .json(&create_task)
            .send()
            .await?;

        let data = resp.bytes().await?;
        let data: CreateTaskResponse = serde_json::from_slice(&data)?;

        match data {
            CreateTaskResponse::Success(task) => {
                Ok(CapMonsterTask {
                    client: &self.client,
                    api_key: &self.api_key,
                    _type: task_type,
                    id: TaskId(task.task_id),
                })
            }
            CreateTaskResponse::Error(err) => {
                match err.error_description {
                    Some(description) => Err(format_err!("{} - {}", err.error_code, description)),
                    None => Err(format_err!("{}", err.error_code))
                }
            }
        }
    }
}

pub struct CapMonsterTask<'a> {
    client: &'a reqwest::Client,
    api_key: &'a str,
    _type: TaskCaptchaType,
    id: TaskId
}

impl CapMonsterTask<'_> {
    pub async fn get_task_result(&self) -> anyhow::Result<GetTaskResultResponse> {
        let payload = GetTaskResultPayload {
            client_key: &self.api_key,
            task_id: *self.id,
        };

        let resp = self.client.post("https://api.capmonster.cloud/getTaskResult").json(&payload).send().await?;
        let data = resp.bytes().await?;

        Ok(serde_json::from_slice(&data)?)
    }

    pub async fn poll_task_result(self) -> anyhow::Result<(), CapMonsterError> {
        let task = self;

        let mut attempts = 0;
        // https://docs.capmonster.cloud/docs/api/methods/get-task-result
        // Limit: 120 requests per task. If the limit is exceeded, the user's account may be temporarily locked.
        let max_attempts = 120;
        let mut output = None;
        while attempts < max_attempts {
            attempts += 1;
            match task.get_task_result().await? {
                GetTaskResultResponse::Success(success) => {}
                GetTaskResultResponse::Processing(processing) => {

                }
                GetTaskResultResponse::Error(error) => {
                    return Err(error.error_code)
                }
            }
        }

        Ok(())
    }
}

mod test {
    use crate::builders::RecaptchaV2Builder;
    use crate::CapMonster;

    async fn main() {
        let mon = CapMonster::new("", None);

        let builder = RecaptchaV2Builder::new().build().unwrap();

        let task = mon.create_task(builder).await.unwrap();
    }
}
