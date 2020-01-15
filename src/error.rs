use thiserror::Error;

#[derive(Error, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Error {
    #[error("no valid response received out of all the allowed retries")]
    RetryLimit,
    #[error("invalid status code ({0})")]
    InvalidStatusCode(u16),
    #[error("the session cookie couldn't be extracted")]
    CookieNotFound,
    #[error("the EPITECH intranet couldn't be reached")]
    UnreachableRemote,
    #[error("internal error")]
    InternalError,
    #[error("parser error: '{0}'")]
    ParserError(String),
    #[error("request error: '{0}'")]
    RequestError(String),
}

impl From<json::Error> for Error {
    fn from(v: json::Error) -> Error {
        Error::ParserError(v.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(v: reqwest::Error) -> Error {
        Error::RequestError(v.to_string())
    }
}
