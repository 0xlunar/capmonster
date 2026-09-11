use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter};

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
    ProxyMissing,
    ProxyNotAuthorised,
    ProxyReadTimeout,
    TaskNotSupported,
    TaskAbsent,
    WrongUserAgent,
    ServiceNotAvailable,
    InvalidTask,
}

impl Display for CapMonsterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CapMonsterError::TaskError(error) => error.fmt(f),
            CapMonsterError::Custom(custom) => f.write_str(&custom)
        }
    }
}

impl Display for TaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Using Debug output for error code to use name instead of the Display impl descriptions.
        write!(f, "{} | {:?} - {}", self.error_id, self.error_code, self.error_description)
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::KeyDoesNotExist => f.write_str("Key does not exist"),
            ErrorCode::ProxyCredentialsInvalidCharacter => f.write_str("Proxy credentials contain invalid character"),
            ErrorCode::ZeroBalance => f.write_str("Zero balance"),
            ErrorCode::TooBigCaptchaFileSize => f.write_str("Image exceeds maximum size (50KB)"),
            ErrorCode::ZeroCaptchaFileSize => f.write_str("Image size less than 100 bytes"),
            ErrorCode::CaptchaIdNotFound => f.write_str("Task does not exist or task expired."),
            ErrorCode::CaptchaUnsolvable => f.write_str("Service failed to solve task"),
            ErrorCode::CaptchaNotReady => f.write_str("Captcha not ready"),
            ErrorCode::IpNotAllowed => f.write_str("Request is not allowed from your ip (configure in dashboard)"),
            ErrorCode::IpBanned => f.write_str("Exceeded request limit with incorrect API Key"),
            ErrorCode::NoSuchMethod => f.write_str("Incorrect task type specified"),
            ErrorCode::TooMuchRequests => f.write_str("Exceeded request limit for getting a response on one task"),
            ErrorCode::DomainNotAllowed => f.write_str("Solving captcha forbidden on domain"),
            ErrorCode::TokenExpired => f.write_str("Additional token expired"),
            ErrorCode::RecaptchaInvalidSiteKey => f.write_str("Invalid websiteKey provided"),
            ErrorCode::RecaptchaInvalidDomain => f.write_str("Domain does not match the specified sitekey or url is in incorrect format"),
            ErrorCode::RecaptchaTimeout => f.write_str("Recaptcha solving time expired"),
            ErrorCode::IpBlocked => f.write_str("IP Blocked due to high number of failed requests"),
            ErrorCode::ProxyConnectRefused => f.write_str("Failed to connect to proxy"),
            ErrorCode::ProxyBanned => f.write_str("Proxy is banned on captcha service"),
            ErrorCode::ProxyMissing => f.write_str("Proxy parameters are missing in required fields or incorrect proxy_type"),
            ErrorCode::ProxyNotAuthorised => f.write_str("Incorrect proxy authorization data"),
            ErrorCode::ProxyReadTimeout => f.write_str("Incorrect proxyAddress or proxyPort causing connection timeout"),
            ErrorCode::TaskNotSupported => f.write_str("Specified task type is unsupported or invalid"),
            ErrorCode::TaskAbsent => f.write_str("Task object missing"),
            ErrorCode::WrongUserAgent => f.write_str("Invalid User Agent was provided"),
            ErrorCode::ServiceNotAvailable => f.write_str("Service is temporarily unavailable"),
            ErrorCode::InvalidTask => f.write_str("Task contains invalid data but is syntactically correct"),
        }
    }
}

impl std::error::Error for ErrorCode {}
impl std::error::Error for TaskError {}
impl std::error::Error for CapMonsterError {}

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
            "ERROR_PROXY_MISSING" => Ok(Self::ProxyMissing),
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
