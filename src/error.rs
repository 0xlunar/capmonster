use std::fmt::Formatter;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, Visitor};

#[derive(Debug)]
pub enum CapMonsterError {
    TaskError(TaskError),
    Custom(String),
}

#[derive(Debug)]
pub struct TaskError {
    pub error_id: i16,
    pub error_code: ErrorCode,
    pub error_description: String,
}

#[derive(Debug)]
pub enum ErrorCode {
    KeyDoesNotExist,
    ProxyCredentialsInvalidCharacter,
    ZeroBalance,
    TooBigCaptchaFileSize,
    ZeroCaptchaFileSize,
    CaptchaIdNotFound,
    CaptchaUnsolvable,
    CaptchaNotReady,
    IpNotAllowed,
    IpBanned,
    NoSuchMethod,
    TooMuchRequests,
    DomainNotAllowed,
    TokenExpired,
    RecaptchaInvalidSiteKey,
    RecaptchaInvalidDomain,
    RecaptchaTimeout,
    IpBlocked,
    ProxyConnectRefused,
    ProxyBanned,
    ProxyNotAuthorised,
    ProxyReadTimeout,
    TaskNotSupported,
    TaskAbsent,
    WrongUserAgent,
    ServiceNotAvailable,
    InvalidTask,
}

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ErrorCodeVisitor)
    }
}

impl TryFrom<&str> for ErrorCode {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ERROR_KEY_DOES_NOT_EXIST" => Ok(Self::KeyDoesNotExist),
            "ERROR_PROXY_CREDENTIALS_INVALID_CHARACTER" => {
                Ok(Self::ProxyCredentialsInvalidCharacter)
            }
            "ERROR_ZERO_BALANCE" => Ok(Self::ZeroBalance),
            "ERROR_TOO_BIG_CAPTCHA_FILESIZE" => Ok(Self::TooBigCaptchaFileSize),
            "ERROR_ZERO_CAPTCHA_FILESIZE" => Ok(Self::ZeroCaptchaFileSize),
            "ERROR_NO_SUCH_CAPCHA_ID" | "WRONG_CAPTCHA_ID" => Ok(Self::CaptchaIdNotFound),
            "ERROR_CAPTCHA_UNSOLVABLE" => Ok(Self::CaptchaUnsolvable),
            "CAPTCHA_NOT_READY" => Ok(Self::CaptchaNotReady),
            "ERROR_IP_NOT_ALLOWED" => Ok(Self::IpNotAllowed),
            "ERROR_IP_BANNED" => Ok(Self::IpBanned),
            "ERROR_NO_SUCH_METHOD" => Ok(Self::NoSuchMethod),
            "ERROR_TOO_MUCH_REQUESTS" => Ok(Self::TooMuchRequests),
            "ERROR_DOMAIN_NOT_ALLOWED" => Ok(Self::DomainNotAllowed),
            "ERROR_TOKEN_EXPIRED" => Ok(Self::TokenExpired),
            "ERROR_RECAPTCHA_INVALID_SITEKEY" => Ok(Self::RecaptchaInvalidSiteKey),
            "ERROR_RECAPTCHA_INVALID_DOMAIN" => Ok(Self::RecaptchaInvalidDomain),
            "ERROR_RECAPTCHA_TIMEOUT" => Ok(Self::RecaptchaTimeout),
            "ERROR_IP_BLOCKED" => Ok(Self::IpBlocked),
            "ERROR_PROXY_CONNECT_REFUSED" => Ok(Self::ProxyConnectRefused),
            "ERROR_PROXY_BANNED" => Ok(Self::ProxyBanned),
            "ERROR_PROXY_NOT_AUTHORISED" => Ok(Self::ProxyNotAuthorised),
            "ERROR_PROXY_READ_TIMEOUT" => Ok(Self::ProxyReadTimeout),
            "ERROR_TASK_NOT_SUPPORTED" => Ok(Self::TaskNotSupported),
            "ERROR_TASK_ABSENT" => Ok(Self::TaskAbsent),
            "ERROR_WRONG_USERAGENT" => Ok(Self::WrongUserAgent),
            "ERROR_SERVICE_NOT_AVAILABLE" => Ok(Self::ServiceNotAvailable),
            "ERROR_INVALID_TASK" => Ok(Self::InvalidTask),
            _ => Err("Unknown error code"),
        }
    }
}

struct ErrorCodeVisitor;

impl<'de> Visitor<'de> for ErrorCodeVisitor {
    type Value = ErrorCode;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        ErrorCode::try_from(v).map_err(E::custom)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(&v)
    }
}