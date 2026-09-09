use crate::{CapMonster, CapMonsterError, Task};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

macro_rules! impl_solver {
    ($solver:ty, $input:ty, $output:ty) => {
        impl CapMonster<$solver> {
            pub async fn create_task(
                &self,
                task: $input,
            ) -> Result<Task<$solver>, CapMonsterError> {
                self.create_task_internal(task).await
            }
        }

        impl Task<$solver> {
            pub async fn get_task_result(&self) -> Result<Option<$output>, CapMonsterError> {
                self.get_task_result_internal().await
            }
            pub async fn wait_for_result(
                &self,
                poll_rate: Option<Duration>,
                max_attempts: Option<usize>,
            ) -> Result<$output, CapMonsterError> {
                self.wait_for_result_internal(poll_rate, max_attempts).await
            }
        }
    };
}

macro_rules! set_key_option {
    ($key:ident, $value:ty) => {
        pub fn $key(mut self, $key: $value) -> Self {
            self.$key = Some($key);
            self
        }
    };
}

macro_rules! impl_task {
    ($lifetime:lifetime, $task:ident, $task_type:literal, {$($additional_static:ident: $additional_value:literal),*$(,)*}, {$($required_key:ident: $required_type:ty),*$(,)*}, {$($optional_key:ident: $optional_type:ty),*$(,)*}) => {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $task<$lifetime> {
            #[serde(rename = "type")]
            _type: &'static str,
            $($additional_static: &'static str,)*
            $($required_key: $required_type,)*
            $($optional_key: Option<$optional_type>,)*
        }

        impl<$lifetime> $task<$lifetime> {
            pub fn new($($required_key: $required_type),*) -> Self {
                Self {
                    _type: $task_type,
                    $($additional_static: $additional_value,)*
                    $($required_key,)*
                    $($optional_key: None,)*
                }
            }

            $(set_key_option!($optional_key, $optional_type);)*
        }
    };
}

//////////////////////////////////////////////

pub struct CloudflareTurnstile;
impl_task!(
    'a,
    CloudflareTurnstileTask,
    "TurnstileTask",
    {},
    {
        website_url: &'a str,
        website_key: &'a str,
    },
    {
        user_agent: &'a str,
        page_action: &'a str,
        data: &'a str,
        proxy_type: &'a str,
        proxy_address: &'a str,
        proxy_port: &'a str,
        proxy_login: &'a str,
        proxy_password: &'a str
    }
);
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudflareTurnstileSolution {
    pub user_agent: String,
    pub token: String,
}

impl_solver!(
    CloudflareTurnstile,
    CloudflareTurnstileTask<'_>,
    CloudflareTurnstileSolution
);

//////////////////////////////////////////////

pub struct RecaptchaV2;
impl_task!(
    'a,
    RecaptchaV2Task,
    "RecaptchaV2Task",
    {},
    {
        website_url: &'a str,
        website_key: &'a str,
    },
    {
        recaptcha_data_s_value: &'a str,
        user_agent: &'a str,
        cookies: &'a str,
        is_invisible: bool,
        proxy_type: &'a str,
        proxy_address: &'a str,
        proxy_port: &'a str,
        proxy_login: &'a str,
        proxy_password: &'a str,
    }
);
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaV2Solution {
    pub g_recaptcha_response: String,
}

impl_solver!(RecaptchaV2, RecaptchaV2Task<'_>, RecaptchaV2Solution);

//////////////////////////////////////////////

pub struct RecaptchaV3;
impl_task!(
    'a,
    RecaptchaV3Task,
    "RecaptchaV3TaskProxyless",
    {},
    {
        website_url: &'a str,
        website_key: &'a str,
    },
    {
        is_enterprise: bool,
        min_score: f64,
        page_action: &'a str,
    }
);
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaV3Solution {
    g_recaptcha_response: String,
}

impl_solver!(RecaptchaV3, RecaptchaV3Task<'_>, RecaptchaV3Solution);

//////////////////////////////////////////////

pub struct RecaptchaV2Enterprise;
#[derive(Serialize)]
pub struct RecaptchaV2EnterpriseTaskPayload<'a> {
    s: &'a str,
}

impl<'a> RecaptchaV2EnterpriseTaskPayload<'a> {
     pub fn new(data: &'a str) -> Self {
         Self {
             s: data,
         }
     }
}

impl_task!(
    'a,
    RecaptchaV2EnterpriseTask,
    "RecaptchaV2EnterpriseTask",
    {},
    {
        website_url: &'a str,
        website_key: &'a str,
    },
    {
        page_action: &'a str,
        enterprise_payload: RecaptchaV2EnterpriseTaskPayload<'a>,
        api_domain: &'a str,
        user_agent: &'a str,
        cookies: &'a str,
        proxy_type: &'a str,
        proxy_address: &'a str,
        proxy_port: &'a str,
        proxy_login: &'a str,
        proxy_password: &'a str,
    }
);
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaV2EnterpriseSolution {
    pub g_recaptcha_response: String,
}

impl_solver!(
    RecaptchaV2Enterprise,
    RecaptchaV2EnterpriseTask<'_>,
    RecaptchaV2EnterpriseSolution
);

//////////////////////////////////////////////

pub struct RecaptchaV3Enterprise;
impl_task!(
    'a,
    RecaptchaV3EnterpriseTask,
    "RecaptchaV3EnterpriseTask",
    {},
    {
        website_url: &'a str,
        website_key: &'a str,
    },
    {
        min_score: f64,
        page_action: &'a str,
    }
);
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaV3EnterpriseSolution {
    pub g_recaptcha_response: String,
}

impl_solver!(
    RecaptchaV3Enterprise,
    RecaptchaV3EnterpriseTask<'_>,
    RecaptchaV3EnterpriseSolution
);

//////////////////////////////////////////////

pub struct GeeTest;
impl_task!(
    'a,
    GeeTestTask,
    "GeeTestTask",
    {},
    {
        website_url: &'a str,
        gt: &'a str,
        challenge: &'a str,
        version: i64,
    },
    {
        geettest_api_server_subdomain: &'a str,
        geetest_get_lib: &'a str,
        user_agent: &'a str,
        proxy_type: &'a str,
        proxy_address: &'a str,
        proxy_port: &'a str,
        proxy_login: &'a str,
        proxy_password: &'a str,
    }
);
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeeTestSolution {
    pub challenge: String,
    pub validate: String,
    pub seccode: String,
}

impl_solver!(GeeTest, GeeTestTask<'_>, GeeTestSolution);

//////////////////////////////////////////////

pub struct CloudflareChallengeToken;
impl_task!(
    'a,
    CloudflareChallengeTokenTask,
    "TurnstileTask",
    {
        cloudflare_task_type: "token"
    },
    {
        website_url: &'a str,
        website_key: &'a str,
        page_action: &'a str,
        user_agent: &'a str,
        data: &'a str,
        page_data: &'a str,
    },
    {
        api_js_url: &'a str,
        proxy_type: &'a str,
        proxy_address: &'a str,
        proxy_port: &'a str,
        proxy_login: &'a str,
        proxy_password: &'a str
    }
);
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudflareChallengeTokenSolution {
    pub user_agent: String,
    pub token: String,
}

impl_solver!(
    CloudflareChallengeToken,
    CloudflareChallengeTokenTask<'_>,
    CloudflareChallengeTokenSolution
);

//////////////////////////////////////////////

pub struct CloudflareChallengeCookie;
impl_task!(
    'a,
    CloudflareChallengeCookieTask,
    "TurnstileTask",
    {
        cloudflare_task_type: "cf_clearance"
    },
    {
        website_url: &'a str,
        website_key: &'a str,
        html_page_base64: &'a str,
        user_agent: &'a str,
        proxy_type: &'a str,
        proxy_address: &'a str,
        proxy_port: &'a str,
        proxy_login: &'a str,
        proxy_password: &'a str,
    },
    {}
);
#[derive(Deserialize)]
pub struct CloudflareChallengeCookieSolution {
    pub cf_clearance: String,
}

impl_solver!(
    CloudflareChallengeCookie,
    CloudflareChallengeCookieTask<'_>,
    CloudflareChallengeCookieSolution
);

//////////////////////////////////////////////

pub struct CloudflareWaitingRoom;
impl_task!(
    'a,
    CloudflareWaitingRoomTask,
    "TurnstileTask",
    {
        cloudflare_task_type: "wait_room"
    },
    {
        website_url: &'a str,
        website_key: &'a str,
        html_page_base64: &'a str,
        user_agent: &'a str,
        proxy_type: &'a str,
        proxy_address: &'a str,
        proxy_port: &'a str,
        proxy_login: &'a str,
        proxy_password: &'a str,
    },
    {}
);
#[derive(Deserialize)]
pub struct CloudflareWaitingRoomSolution {
    pub cf_clearance: String,
}

impl_solver!(
    CloudflareWaitingRoom,
    CloudflareWaitingRoomTask<'_>,
    CloudflareWaitingRoomSolution
);

//////////////////////////////////////////////

pub struct DataDome;
#[derive(Serialize)]
pub struct DataDomeTaskMetadata<'a> {
    captcha_url: &'a str,
    datadome_cookie: &'a str,
    datadome_version: Option<&'a str>,
}

impl<'a> DataDomeTaskMetadata<'a> {
    pub fn new(captcha_url: &'a str, datadome_cookie: &'a str) -> Self {
        Self {
            captcha_url,
            datadome_cookie,
            datadome_version: None,
        }
    }

    set_key_option!(datadome_version, &'a str);
}

impl_task!(
    'a,
    DataDomeTask,
    "CustomTask",
    {
        class: "DataDome"
    },
    {
        website_url: &'a str,
        metadata: DataDomeTaskMetadata<'a>,
        proxy_type: &'a str,
        proxy_address: &'a str,
        proxy_port: &'a str,
        proxy_login: &'a str,
        proxy_password: &'a str,
    },
    {
        user_agent: &'a str,
    }
);
#[derive(Deserialize)]
pub struct DataDomeSolution {
    pub domains: HashMap<String, DataDomeSolutionDomain>,
    pub url: Option<String>,
    pub fingerprint: Option<String>,
    pub headers: Option<String>,
    pub data: Option<Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataDomeSolutionDomain {
    pub cookies: DataDomeSolutionCookie,
    pub local_storage: Option<Value>,
}

#[derive(Deserialize)]
pub struct DataDomeSolutionCookie {
    pub datadome: String,
}

impl_solver!(DataDome, DataDomeTask<'_>, DataDomeSolution);

//////////////////////////////////////////////

pub struct Basilisk;
impl_task!(
    'a,
    BasiliskTask,
    "CustomTask",
    {
        class: "Basilisk"
    },
    {
        website_url: &'a str,
        website_key: &'a str,
    },
    {
        user_agent: &'a str,
        proxy_type: &'a str,
        proxy_address: &'a str,
        proxy_port: &'a str,
        proxy_login: &'a str,
        proxy_password: &'a str,
    }
);
#[derive(Deserialize)]
pub struct BasiliskSolution {
    pub data: BasiliskSolutionData,
    pub headers: BasiliskSolutionHeaders,
}
#[derive(Deserialize)]
pub struct BasiliskSolutionData {
    pub captcha_response: String,
}
#[derive(Deserialize)]
pub struct BasiliskSolutionHeaders {
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
}

impl_solver!(
    Basilisk,
    BasiliskTask<'_>,
    BasiliskSolution
);


//////////////////////////////////////////////

pub struct TenDI;
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenDITaskMetadata<'a> {
    captcha_url: Option<&'a str>,
}

impl<'a> TenDITaskMetadata<'a> {
    pub fn new() -> Self {
        Self {
            captcha_url: None,
        }
    }

    set_key_option!(captcha_url, &'a str);
}

impl_task!(
    'a,
    TenDITask,
    "CustomTask",
    {
        class: "TenDI"
    },
    {
        website_url: &'a str,
        website_key: &'a str,
    },
    {
        metadata: TenDITaskMetadata<'a>,
        user_agent: &'a str,
        proxy_type: &'a str,
        proxy_address: &'a str,
        proxy_port: &'a str,
        proxy_login: &'a str,
        proxy_password: &'a str,
    }
);
#[derive(Deserialize)]
pub struct TenDISolution {
    pub data: BasiliskSolutionData,
    pub headers: BasiliskSolutionHeaders,
}
#[derive(Deserialize)]
pub struct TenDISolutionData {
    pub randstr: String,
    pub ticket: String,
}
#[derive(Deserialize)]
pub struct TenDISolutionHeaders {
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
}

impl_solver!(
    TenDI,
    TenDITask<'_>,
    TenDISolution
);

//////////////////////////////////////////////

pub struct AmazonAWSWAF;
#[derive(Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum AmazonAWSWAFTaskData<'a> {
    Captcha {
        website_key: &'a str,
        user_agent: &'a str,
        captcha_script: &'a str,
    },
    CaptchaAndChallenge {
        website_key: &'a str,
        challenge_script: &'a str,
        captcha_script: &'a str,
        context: &'a str,
        iv: &'a str,
    },
    Challenge {
        challenge_script: &'a str,
        context: &'a str,
        iv: &'a str,
    },
}

impl<'a> AmazonAWSWAFTaskData<'a> {
    pub fn captcha(website_key: &'a str, captcha_script: &'a str, user_agent: &'a str) -> Self {
        Self::Captcha {
            website_key,
            user_agent,
            captcha_script,
        }
    }

    pub fn captcha_and_challenge(website_key: &'a str, challenge_script: &'a str, captcha_script: &'a str, context: &'a str, iv: &'a str) -> Self {
        Self::CaptchaAndChallenge {
            website_key,
            challenge_script,
            captcha_script,
            context,
            iv,
        }
    }

    pub fn challenge(challenge_script: &'a str) -> Self {
        Self::Challenge {
            challenge_script,
            context: "",
            iv: "",
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmazonAWSWAFTask<'a> {
    #[serde(rename = "type")]
    _type: &'static str,
    cookie_solution: bool,
    website_url: &'a str,
    #[serde(flatten)]
    task_data: AmazonAWSWAFTaskData<'a>,
    proxy_type: Option<&'a str>,
    proxy_address: Option<&'a str>,
    proxy_port: Option<&'a str>,
    proxy_login: Option<&'a str>,
    proxy_password: Option<&'a str>,
}

impl<'a> AmazonAWSWAFTask<'a> {
    pub fn new(website_url: &'a str, data: AmazonAWSWAFTaskData<'a>) -> Self {
        Self {
            _type: "AmazonTask",
            cookie_solution: true,
            website_url,
            task_data: data,
            proxy_type: None,
            proxy_address: None,
            proxy_port: None,
            proxy_login: None,
            proxy_password: None,
        }
    }

    set_key_option!(proxy_type, &'a str);
    set_key_option!(proxy_address, &'a str);
    set_key_option!(proxy_port, &'a str);
    set_key_option!(proxy_login, &'a str);
    set_key_option!(proxy_password, &'a str);
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmazonAWSWAFSolution {
    pub cookies: AmazonAWSWAFSolutionCookies,
    pub user_agent: BasiliskSolutionHeaders,
}

#[derive(Deserialize)]
pub struct AmazonAWSWAFSolutionCookies {
    #[serde(rename = "aws-waf-token")]
    pub aws_waf_token: String,
}

impl_solver!(
    AmazonAWSWAF,
    AmazonAWSWAFTask<'_>,
    AmazonAWSWAFSolution
);

//////////////////////////////////////////////

pub struct Binance;
impl_task!(
    'a,
    BinanceTask,
    "BinanceTask",
    {},
    {
        website_url: &'a str,
        website_key: &'a str,
        validate_id: &'a str,
    },
    {
        user_agent: &'a str,
        proxy_type: &'a str,
        proxy_address: &'a str,
        proxy_port: &'a str,
        proxy_login: &'a str,
        proxy_password: &'a str,
    }
);
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceSolution {
    pub token: String,
    pub user_agent: String,
}

impl_solver!(Binance, BinanceTask<'_>, BinanceSolution);

//////////////////////////////////////////////

pub struct Imperva;
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImpervaTaskMetadata<'a> {
    incapsula_script_url: &'a str,
    incapsula_cookies: &'a str,
    reese84_url_endpoint: Option<&'a str>,
}

impl<'a> ImpervaTaskMetadata<'a> {
    pub fn new(script_url: &'a str, cookies: &'a str) -> Self {
        Self {
            incapsula_script_url: script_url,
            incapsula_cookies: cookies,
            reese84_url_endpoint: None,
        }
    }

    set_key_option!(reese84_url_endpoint, &'a str);
}

impl_task!(
    'a,
    ImpervaTask,
    "CustomTask",
    {
        class: "Imperva",
    },
    {
        website_url: &'a str,
        website_key: &'a str,
        metadata: ImpervaTaskMetadata<'a>,
        proxy_type: &'a str,
        proxy_address: &'a str,
        proxy_port: &'a str,
        proxy_login: &'a str,
        proxy_password: &'a str,
    },
    {
        user_agent: &'a str,
    }
);
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImpervaSolution {
    pub domains: HashMap<String, ImpervaSolutionDomain>,
    pub user_agent: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImpervaSolutionDomain {
    pub cookies: ImpervaSolutionCookie
}

#[derive(Deserialize)]
pub struct ImpervaSolutionCookie {
    pub ___utmvc: String,
}

impl_solver!(Imperva, ImpervaTask<'_>, ImpervaSolution);

//////////////////////////////////////////////