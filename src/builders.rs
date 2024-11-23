use crate::types::*;
use anyhow::format_err;
use serde::Serialize;

pub trait Builder<T: Serialize> {
    fn get_type(&self) -> TaskCaptchaType;
    fn build(self) -> anyhow::Result<T>;
}

pub struct RecaptchaV2Builder {
    proxy: RecaptchaV2ProxyType,
    enterprise: RecaptchaV2Type,
    website_url: Option<String>,
    website_key: Option<String>,
    recaptcha_data_s_value: Option<String>,
    user_agent: Option<String>,
    cookies: Option<String>,
    is_invisible: Option<bool>,
    proxy_data: Option<TaskProxy>,
}
pub struct RecaptchaV3Builder {
    website_url: Option<String>,
    website_key: Option<String>,
    min_score: Option<f64>,
    page_action: Option<String>,
}
pub struct HCaptchaBuilder {
    website_url: Option<String>,
    website_key: Option<String>,
    is_invisible: Option<bool>,
    data: Option<String>,
    user_agent: Option<String>,
    cookies: Option<String>,
    fallback_to_actual_ua: Option<bool>,
    proxy_data: Option<TaskProxy>,
}
pub struct GeeTestBuilder;
pub struct TurnstileBuilder;
pub struct ComplexImageRecognitionBuilder;
pub struct ComplexImageRecaptchaBuilder;
pub struct ComplexImageHCaptchaBuilder;
pub struct ImageToTextBuilder;
pub struct DataDomeBuilder;
pub struct TenDIBuilder;
pub struct AmazonBuilder;
pub struct ImpervaBuilder;

impl RecaptchaV2Builder {
    pub fn new() -> Self {
        Self {
            proxy: RecaptchaV2ProxyType::Proxyless,
            enterprise: RecaptchaV2Type::Standard,
            website_url: None,
            website_key: None,
            recaptcha_data_s_value: None,
            user_agent: None,
            cookies: None,
            is_invisible: None,
            proxy_data: None,
        }
    }

    pub fn set_proxy(&mut self, proxy: TaskProxy) -> &mut Self {
        self.proxy = RecaptchaV2ProxyType::Proxy;
        self.proxy_data = Some(proxy);
        self
    }
    pub fn set_proxyless(&mut self) -> &mut Self {
        self.proxy = RecaptchaV2ProxyType::Proxyless;
        self.proxy_data = None;
        self
    }
    pub fn set_enterprise(&mut self) -> &mut Self {
        self.enterprise = RecaptchaV2Type::Enterprise;
        self
    }
    pub fn set_standard(&mut self) -> &mut Self {
        self.enterprise = RecaptchaV2Type::Standard;
        self
    }

    pub fn set_website_url(&mut self, website_url: &str) -> &mut Self {
        self.website_url = Some(website_url.to_string());
        self
    }

    pub fn unset_website_url(&mut self) -> &mut Self {
        self.website_url = None;
        self
    }

    pub fn set_website_key(&mut self, website_key: &str) -> &mut Self {
        self.website_key = Some(website_key.to_string());
        self
    }

    pub fn unset_website_key(&mut self) -> &mut Self {
        self.website_key = None;
        self
    }

    pub fn set_recaptcha_data_s_value(&mut self, recaptcha_data_s_value: String) -> &mut Self {
        self.recaptcha_data_s_value = Some(recaptcha_data_s_value);
        self
    }

    pub fn unset_recaptcha_data_s_value(&mut self) -> &mut Self {
        self.recaptcha_data_s_value = None;
        self
    }

    pub fn set_user_agent(&mut self, user_agent: String) -> &mut Self {
        self.user_agent = Some(user_agent);
        self
    }
    pub fn unset_user_agent(&mut self) -> &mut Self {
        self.user_agent = None;
        self
    }

    pub fn set_cookies(&mut self, cookies: String) -> &mut Self {
        self.cookies = Some(cookies);
        self
    }

    pub fn unset_cookies(&mut self) -> &mut Self {
        self.cookies = None;
        self
    }

    pub fn set_is_invisible(&mut self) -> &mut Self {
        self.is_invisible = Some(true);
        self
    }

    pub fn unset_is_invisible(&mut self) -> &mut Self {
        self.is_invisible = None;
        self
    }

    pub fn build(self) -> anyhow::Result<RecaptchaV2Task> {
        let mut _type = "RecaptchaV2".to_string();
        match self.enterprise {
            RecaptchaV2Type::Standard => _type.push_str("Task"),
            RecaptchaV2Type::Enterprise => _type.push_str("EnterpriseTask"),
        };

        let proxy = match &self.proxy {
            RecaptchaV2ProxyType::Proxyless => {
                _type.push_str("Proxyless");
                None
            }
            RecaptchaV2ProxyType::Proxy => match self.proxy_data {
                Some(data) => Some(data),
                None => return Err(format_err!("Proxy Solver selected but missing proxy data")),
            },
        };

        let website_url = match self.website_url {
            Some(url) => url,
            None => return Err(format_err!("Missing website_url")),
        };

        let website_key = match self.website_key {
            Some(key) => key,
            None => return Err(format_err!("Missing website_key")),
        };

        Ok(RecaptchaV2Task {
            _type,
            website_url,
            website_key,
            recaptcha_data_s_value: self.recaptcha_data_s_value,
            user_agent: self.user_agent,
            cookies: self.cookies,
            is_invisible: self.is_invisible,
            proxy,
        })
    }
}

impl Builder<RecaptchaV2Task> for RecaptchaV2Builder {
    fn get_type(&self) -> TaskCaptchaType {
        TaskCaptchaType::Recaptcha
    }

    fn build(self) -> anyhow::Result<RecaptchaV2Task> {
        let mut _type = "RecaptchaV2".to_string();
        match self.enterprise {
            RecaptchaV2Type::Standard => _type.push_str("Task"),
            RecaptchaV2Type::Enterprise => _type.push_str("EnterpriseTask"),
        };

        let proxy = match &self.proxy {
            RecaptchaV2ProxyType::Proxyless => {
                _type.push_str("Proxyless");
                None
            }
            RecaptchaV2ProxyType::Proxy => match self.proxy_data {
                Some(data) => Some(data),
                None => return Err(format_err!("Proxy Solver selected but missing proxy data")),
            },
        };

        let website_url = match self.website_url {
            Some(url) => url,
            None => return Err(format_err!("Missing website_url")),
        };

        let website_key = match self.website_key {
            Some(key) => key,
            None => return Err(format_err!("Missing website_key")),
        };

        Ok(RecaptchaV2Task {
            _type,
            website_url,
            website_key,
            recaptcha_data_s_value: self.recaptcha_data_s_value,
            user_agent: self.user_agent,
            cookies: self.cookies,
            is_invisible: self.is_invisible,
            proxy,
        })
    }
}

impl RecaptchaV3Builder {
    pub fn new() -> Self {
        Self {
            website_url: None,
            website_key: None,
            min_score: None,
            page_action: None,
        }
    }

    pub fn set_website_url(&mut self, website_url: &str) -> &mut Self {
        self.website_url = Some(website_url.to_string());
        self
    }

    pub fn unset_website_url(&mut self) -> &mut Self {
        self.website_url = None;
        self
    }

    pub fn set_website_key(&mut self, website_key: &str) -> &mut Self {
        self.website_key = Some(website_key.to_string());
        self
    }

    pub fn unset_website_key(&mut self) -> &mut Self {
        self.website_key = None;
        self
    }

    pub fn set_min_score(&mut self, min_score: f64) -> &mut Self {
        let min_score = BetweenPointOneAndPointNine::new(min_score);
        self.min_score = Some(*min_score);
        self
    }

    pub fn unset_min_score(&mut self) -> &mut Self {
        self.min_score = None;
        self
    }

    pub fn set_page_action(&mut self, page_action: &str) -> &mut Self {
        self.page_action = Some(page_action.to_string());
        self
    }

    pub fn unset_page_action(&mut self) -> &mut Self {
        self.page_action = None;
        self
    }

    pub fn build(self) -> anyhow::Result<RecaptchaV3Task> {
        let website_url = match self.website_url {
            Some(url) => url,
            None => return Err(format_err!("website_url not set")),
        };

        let website_key = match self.website_key {
            Some(key) => key,
            None => return Err(format_err!("website_key not set")),
        };

        Ok(RecaptchaV3Task {
            _type: "RecaptchaV3TaskProxyless".to_string(),
            website_url,
            website_key,
            min_score: self.min_score,
            page_action: self.page_action,
        })
    }
}

impl HCaptchaBuilder {
    pub fn new() -> Self {
        Self {
            website_url: None,
            website_key: None,
            is_invisible: None,
            data: None,
            user_agent: None,
            cookies: None,
            fallback_to_actual_ua: None,
            proxy_data: None,
        }
    }

    pub fn set_website_url(&mut self, website_url: &str) -> &mut Self {
        self.website_url = Some(website_url.to_string());
        self
    }

    pub fn unset_website_url(&mut self) -> &mut Self {
        self.website_url = None;
        self
    }

    pub fn set_website_key(&mut self, website_url: &str) -> &mut Self {
        self.website_key = Some(website_url.to_string());
        self
    }

    pub fn unset_website_key(&mut self) -> &mut Self {
        self.website_key = None;
        self
    }

    pub fn set_is_invisible(&mut self) -> &mut Self {
        self.is_invisible = Some(true);
        self
    }

    pub fn unset_is_invisible(&mut self) -> &mut Self {
        self.is_invisible = None;
        self
    }

    pub fn set_data(&mut self, data: &str) -> &mut Self {
        self.data = Some(data.to_string());
        self
    }

    pub fn unset_data(&mut self) -> &mut Self {
        self.data = None;
        self
    }

    pub fn set_user_agent(&mut self, user_agent: &str) -> &mut Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    pub fn unset_user_agent(&mut self) -> &mut Self {
        self.user_agent = None;
        self
    }

    pub fn set_cookies(&mut self, cookies: &str) -> &mut Self {
        self.cookies = Some(cookies.to_string());
        self
    }

    pub fn unset_cookies(&mut self) -> &mut Self {
        self.cookies = None;
        self
    }

    pub fn set_fallback_to_actual_ua(&mut self) -> &mut Self {
        self.fallback_to_actual_ua = Some(true);
        self
    }

    pub fn unset_fallback_to_actual_ua(&mut self) -> &mut Self {
        self.fallback_to_actual_ua = None;
        self
    }

    pub fn set_proxy(&mut self, proxy: TaskProxy) -> &mut Self {
        self.proxy_data = Some(proxy);
        self
    }

    pub fn unset_proxy(&mut self) -> &mut Self {
        self.fallback_to_actual_ua = None;
        self
    }

    pub fn build(self) -> anyhow::Result<HCaptchaTask> {
        let website_url = match self.website_url {
            Some(url) => url,
            None => return Err(format_err!("website_url not set")),
        };

        let website_key = match self.website_key {
            Some(key) => key,
            None => return Err(format_err!("website_key not set")),
        };

        let proxy = match self.proxy_data {
            Some(proxy) => proxy,
            None => return Err(format_err!("proxy not set")),
        };

        Ok(HCaptchaTask {
            _type: "HCaptchaTask".to_string(),
            website_url,
            website_key,
            is_invisible: self.is_invisible,
            data: self.data,
            user_agent: self.user_agent,
            cookies: self.cookies,
            fallback_to_actual_ua: self.fallback_to_actual_ua,
            proxy,
        })
    }
}
