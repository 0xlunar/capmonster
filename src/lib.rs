pub mod error;
pub mod solver;

use crate::error::{CapMonsterError, ErrorCode, TaskError};
use log::debug;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;
use wreq::Client;

const BASE_URL: &'static str = "https://api.capmonster.cloud";
const POLL_RATE: Duration = Duration::from_secs(2);
const MAX_ATTEMPTS: usize = 20;

pub struct CapMonster<T> {
    client: Client,
    client_key: Arc<str>,
    callback_url: Option<Arc<str>>,
    _solver: PhantomData<T>,
}

pub struct Task<T> {
    id: u64,
    client: Client,
    client_key: Arc<str>,
    _type: PhantomData<T>,
}

impl<T> CapMonster<T> {
    pub fn new(client: Client, client_key: impl Into<Arc<str>>, _solver: T) -> Self {
        Self {
            client,
            client_key: client_key.into(),
            callback_url: None,
            _solver: PhantomData::default(),
        }
    }

    pub fn set_callback_url(&mut self, callback_url: impl Into<Arc<str>>) -> &mut Self {
        self.callback_url = Some(callback_url.into());
        self
    }

    async fn create_task_internal(&self, task: impl Serialize) -> Result<Task<T>, CapMonsterError> {
        let endpoint = format!("{}{}", BASE_URL, "/createTask");

        let data = serde_json::json!({
            "clientKey": &*self.client_key,
            "task": task,
            "callbackUrl": self.callback_url.as_ref().map(|url| &**url)
        });

        let response = self
            .client
            .post(endpoint)
            .json(&data)
            .send()
            .await
            .map_err(|err| CapMonsterError::Custom(err.to_string()))?;

        if !response.status().is_success() {
            return Err(CapMonsterError::Custom(format!(
                "Got status {}",
                response.status()
            )));
        }

        #[derive(Deserialize)]
        struct CreateTaskResponse {
            error_id: i16,
            error_code: Option<ErrorCode>,
            error_description: Option<String>,
            task_id: u64,
        }

        let body = response
            .json::<CreateTaskResponse>()
            .await
            .map_err(|err| CapMonsterError::Custom(err.to_string()))?;
        if body.error_id != 0 {
            Err(CapMonsterError::TaskError(TaskError {
                error_id: body.error_id,
                error_description: body.error_description.unwrap_or_else(|| "".to_string()),
                error_code: body
                    .error_code
                    .unwrap_or_else(|| ErrorCode::ServiceNotAvailable),
            }))
        } else {
            Ok(Task {
                id: body.task_id,
                client: self.client.clone(),
                client_key: self.client_key.clone(),
                _type: PhantomData::default(),
            })
        }
    }
}

impl<T> Task<T> {
    async fn get_task_result_internal<Output: DeserializeOwned>(
        &self,
    ) -> Result<Option<Output>, CapMonsterError> {
        let endpoint = format!("{}{}", BASE_URL, "/getTaskResult");

        let data = serde_json::json!({
            "clientKey": &*self.client_key,
            "taskId": self.id,
        });

        let response = self
            .client
            .post(endpoint)
            .json(&data)
            .send()
            .await
            .map_err(|err| CapMonsterError::Custom(err.to_string()))?;

        #[derive(Deserialize)]
        struct GetTaskResultOutput<O> {
            error_code: Option<ErrorCode>,
            error_description: Option<String>,
            error_id: i16,
            status: String,
            solution: Option<O>,
        }

        let body = response
            .json::<GetTaskResultOutput<Output>>()
            .await
            .map_err(|err| CapMonsterError::Custom(err.to_string()))?;
        if body.error_id != 0 {
            Err(CapMonsterError::TaskError(TaskError {
                error_id: body.error_id,
                error_description: body.error_description.unwrap_or_else(|| "".to_string()),
                error_code: body
                    .error_code
                    .unwrap_or_else(|| ErrorCode::ServiceNotAvailable),
            }))
        } else if body.status == "ready" {
            Ok(body.solution)
        } else {
            Ok(None)
        }
    }
    async fn wait_for_result_internal<Output: DeserializeOwned>(
        &self,
        poll_rate: Option<Duration>,
        max_attempts: Option<usize>,
    ) -> Result<Output, CapMonsterError> {
        let mut poll = tokio::time::interval(poll_rate.unwrap_or_else(|| POLL_RATE));
        let mut count = 0;
        while count < max_attempts.unwrap_or_else(|| MAX_ATTEMPTS) {
            count += 1;
            debug!("Solve attempt [{count}] ID: [{}]", self.id);
            poll.tick().await;
            if let Some(solution) = self.get_task_result_internal().await? {
                return Ok(solution);
            }
        }

        Err(CapMonsterError::Custom("Exceeded max attempts".to_string()))
    }
}

mod example {
    use crate::CapMonster;
    use crate::solver::{RecaptchaV2, RecaptchaV2Task};
    use wreq::Client;

    pub async fn recaptcha_v2() {
        let client = Client::new();
        let solver = CapMonster::new(client, "abcdef", RecaptchaV2);

        let task_data = RecaptchaV2Task::new("", "");
        let task = solver
            .create_task(task_data)
            .await
            .expect("Failed to create task");
        let solution = task
            .wait_for_result(None, None)
            .await
            .expect("Failed to get solution");
        println!("Solution: {}", solution.g_recaptcha_response);
    }
}
