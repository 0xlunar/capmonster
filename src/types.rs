use serde::{Deserialize, Serialize};
use std::ops::Deref;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub enum CapMonsterError {
    #[serde(deserialize = "ERROR_KEY_DOES_NOT_EXIST")]
    KeyDoesNotExist,
    #[serde(deserialize = "ERROR_ZERO_BALANCE")]
    ZeroBalance,
    #[serde(deserialize = "ERROR_TOO_BIG_CAPTCHA_FILESIZE")]
    TooBigCaptchaFileSize,
    #[serde(deserialize = "ERROR_ZERO_CAPTCHA_FILESIZE")]
    ZeroCaptchaFileSize,
    #[serde(deserialize = "ERROR_NO_SUCH_CAPCHA_ID")]
    NoSuchCaptchaID,
    #[serde(deserialize = "WRONG_CAPTCHA_ID")]
    WrongCaptchaID,
    #[serde(deserialize = "ERROR_CAPTCHA_UNSOLVABLE")]
    CaptchaUnsolvable,
    #[serde(deserialize = "CAPTCHA_NOT_READY")]
    CaptchaNotReady,
    #[serde(deserialize = "ERROR_IP_NOT_ALLOWED")]
    IPNotAllowed,
    #[serde(deserialize = "ERROR_IP_BANNED")]
    IPBanned,
    #[serde(deserialize = "ERROR_NO_SUCH_METHOD")]
    NoSuchMethod,
    #[serde(deserialize = "ERROR_TOO_MUCH_REQUESTS")]
    TooMuchRequests,
    #[serde(deserialize = "ERROR_DOMAIN_NOT_ALLOWED")]
    DomainNotAllowed,
    #[serde(deserialize = "ERROR_TOKEN_EXPIRED")]
    TokenExpired,
    #[serde(deserialize = "ERROR_NO_SLOT_AVAILABLE")]
    NoSlotAvailable,
    #[serde(deserialize = "ERROR_RECAPTCHA_INVALID_SITEKEY")]
    RecaptchaInvalidSiteKey,
    #[serde(deserialize = "ERROR_RECAPTCHA_INVALID_DOMAIN")]
    RecaptchaInvalidDomain,
    #[serde(deserialize = "ERROR_RECAPTCHA_TIMEOUT")]
    RecaptchaTimeout,
    #[serde(deserialize = "ERROR_IP_BLOCKED")]
    IpBlocked,
    #[serde(deserialize = "ERROR_PROXY_CONNECT_REFUSED")]
    ProxyConnectRefused,
    #[serde(deserialize = "ERROR_PROXY_BANNED")]
    ProxyBanned,
    #[serde(deserialize = "ERROR_TASK_NOT_SUPPORTED")]
    TaskNotSupported,
    #[serde(deserialize = "ERROR_TASK_ABSENT")]
    TaskAbsent,
    #[serde(deserialize = "ERROR_WRONG_USERAGENT")]
    WrongUserAgent
}

#[derive(Serialize)]
pub(crate) struct CreateTask<T: Serialize> {
    #[serde(rename = "clientKey")]
    pub(crate) client_key: String,
    pub(crate) task: T,
    #[serde(rename = "callbackUrl", skip_serializing_if = "Option::is_none")]
    pub(crate) callback_url: Option<String>
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum CreateTaskResponse {
    Success(CreateTaskSuccess),
    Error(CreateTaskError)
}

#[derive(Deserialize, Debug)]
pub(crate) struct CreateTaskSuccess {
    pub(crate) error_id: u8,
    pub(crate) task_id: u64,
}

#[derive(Deserialize, Debug)]
pub(crate) struct CreateTaskError {
    pub(crate) error_id: u8,
    pub(crate) error_code: CapMonsterError,
    pub(crate) error_description: Option<String>,
    pub(crate) task_id: u64,
}

pub struct TaskId(u64);

#[derive(Debug)]
pub(crate) enum TaskCaptchaType {
    Recaptcha,
    GeeTest,
    Turnstile,
    ComplexImageRecognition,
    ComplexImageRecaptcha,
    ImageToText,
    DataDome,
    TenDI,
    Amazon
}

#[derive(Deserialize, Debug)]
pub(crate) enum GetTaskResultResponse {
    Success(GetTaskResultSuccess),
    Processing(GetTaskResultProcessing),
    Error(GetTaskResultError)
}

#[derive(Deserialize, Debug)]
pub(crate) enum GetTaskResultStatus {
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "ready")]
    Ready
}

#[derive(Deserialize, Debug)]
pub(crate) struct GetTaskResultSuccess {
    pub(crate) error_id: u8,
    pub(crate) status: GetTaskResultStatus::Ready,
    pub(crate) solution: Value
}

#[derive(Deserialize, Debug)]
pub(crate) struct GetTaskResultError {
    pub(crate) error_id: u8,
    pub(crate) error_code: CapMonsterError,
    pub(crate) error_description: Option<String>,
    pub(crate) status: GetTaskResultStatus,
}

#[derive(Deserialize, Debug)]
pub(crate) struct GetTaskResultProcessing {
    pub(crate) error_id: u8,
    pub(crate) status: GetTaskResultStatus::Processing,
}

#[derive(Serialize, Debug)]
pub(crate) struct GetTaskResultPayload<'a> {
    pub(crate) client_key: &'a str,
    pub(crate) task_id: u64,
}

#[derive(Default)]
pub(crate) enum RecaptchaV2ProxyType {
    #[default]
    Proxyless,
    Proxy,
}
#[derive(Default)]
pub(crate) enum RecaptchaV2Type {
    #[default]
    Standard,
    Enterprise,
}

#[derive(Serialize)]
pub struct RecaptchaV2Task {
    #[serde(rename = "type")]
    pub(crate) _type: String,
    #[serde(rename = "websiteURL")]
    pub(crate) website_url: String,
    #[serde(rename = "websiteKey")]
    pub(crate) website_key: String,
    #[serde(
        rename = "recaptchaDataSValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) recaptcha_data_s_value: Option<String>,
    #[serde(rename = "userAgent", skip_serializing_if = "Option::is_none")]
    pub(crate) user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cookies: Option<String>,
    #[serde(rename = "isInvisible", skip_serializing_if = "Option::is_none")]
    pub(crate) is_invisible: Option<bool>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub(crate) proxy: Option<TaskProxy>,
}

#[derive(Serialize)]
pub struct TaskProxy {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    #[serde(rename = "proxyAddress")]
    pub proxy_address: String,
    #[serde(rename = "proxyPort")]
    pub proxy_port: u16,
    #[serde(rename = "proxyLogin", skip_serializing_if = "Option::is_none")]
    pub proxy_login: Option<String>,
    #[serde(rename = "proxyPassword", skip_serializing_if = "Option::is_none")]
    pub proxy_password: Option<String>,
}

#[derive(Serialize)]
pub struct RecaptchaV3Task {
    #[serde(rename = "type")]
    pub(crate) _type: String,
    #[serde(rename = "websiteURL")]
    pub(crate) website_url: String,
    #[serde(rename = "websiteKey")]
    pub(crate) website_key: String,
    #[serde(rename = "minScore", skip_serializing_if = "Option::is_none")]
    pub(crate) min_score: Option<f64>,
    #[serde(rename = "pageAction", skip_serializing_if = "Option::is_none")]
    pub(crate) page_action: Option<String>,
}

#[derive(Serialize)]
pub struct HCaptchaTask {
    #[serde(rename = "type")]
    pub(crate) _type: String,
    #[serde(rename = "websiteURL")]
    pub(crate) website_url: String,
    #[serde(rename = "websiteKey")]
    pub(crate) website_key: String,
    #[serde(rename = "isInvisible", skip_serializing_if = "Option::is_none")]
    pub(crate) is_invisible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<String>,
    #[serde(rename = "userAgent", skip_serializing_if = "Option::is_none")]
    pub(crate) user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cookies: Option<String>,
    #[serde(rename = "fallbackToActualUA", skip_serializing_if = "Option::is_none")]
    pub(crate) fallback_to_actual_ua: Option<bool>,
    #[serde(flatten)]
    pub(crate) proxy: TaskProxy,
}

pub(crate) struct BetweenPointOneAndPointNine {
    num: f64,
}
impl BetweenPointOneAndPointNine {
    pub(crate) fn new(num: f64) -> Self {
        let num = num.clamp(0.1, 0.9);
        Self { num }
    }
}

impl Deref for BetweenPointOneAndPointNine {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.num
    }
}

impl Deref for TaskId {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}