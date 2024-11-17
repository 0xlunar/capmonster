use num_traits::Float;
use num_traits::real::Real;
use serde::Serialize;

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
    #[serde(rename = "proxyLogin")]
    pub proxy_login: Option<String>,
    #[serde(rename = "proxyPassword")]
    pub proxy_password: Option<String>,
}

#[derive(Serialize)]
pub struct RecaptchaV3Task {
    #[serde(rename = "type")]
    _type: String,
    #[serde(rename = "websiteURL")]
    website_url: String,
    #[serde(rename = "websiteKey")]
    website_key: String,
    #[serde(rename = "minScore")]
    min_score: Option<f64>,
    #[serde(rename = "pageAction")]
    page_action: Option<String>
}



pub(crate) struct BetweenZeroAndOne {
    num: f64,
}
impl BetweenZeroAndOne {
    pub(crate) fn new(num: f64) -> Self {
        let num = num.clamp(0.0, 1.0);
        Self { num }
    }
}
