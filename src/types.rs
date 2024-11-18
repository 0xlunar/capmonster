use serde::Serialize;
use std::ops::Deref;

#[derive(Serialize)]
pub struct CreateTask<T: Serialize> {
    #[serde(rename = "clientKey")]
    client_key: String,
    task: T,
    #[serde(rename = "callbackUrl", skip_serializing_if = "Option::is_none")]
    callback_url: Option<String>
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
