use reqwest;
use serde_json;

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::marker::{Send, Sync};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum EpitechClientError {
    RetryLimit,
    InvalidStatusCode(u16),
    CookieNotFound,
    UnreachableRemote,
    InternalError,
    ParserError(String),
    RequestError(String),
}

impl From<serde_json::Error> for EpitechClientError {
    fn from(v: serde_json::Error) -> EpitechClientError {
        EpitechClientError::ParserError(String::from(v.description()))
    }
}

impl From<reqwest::Error> for EpitechClientError {
    fn from(v: reqwest::Error) -> EpitechClientError {
        EpitechClientError::RequestError(String::from(v.description()))
    }
}

impl Error for EpitechClientError {
    fn description(&self) -> &str {
        match self {
            EpitechClientError::RetryLimit => {
                "No valid response received out of all the allowed retries."
            }
            EpitechClientError::InvalidStatusCode(_) => "Invalid status code",
            EpitechClientError::CookieNotFound => "The session cookie couldn't be extracted",
            EpitechClientError::UnreachableRemote => "The EPITECH intranet couldn't be reached",
            EpitechClientError::InternalError => "An internal error happened",
            EpitechClientError::ParserError(_) => "A parsing error happened",
            EpitechClientError::RequestError(_) => "A request error happened",
        }
    }
}

impl Display for EpitechClientError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            EpitechClientError::InvalidStatusCode(code) => {
                write!(f, "Invalid status code ({})", code)
            }
            EpitechClientError::ParserError(desc) => write!(f, "A parsing error happened ({})", desc),
            EpitechClientError::RequestError(desc) => write!(f, "A request error happened ({})", desc),
            _ => write!(f, "{}", self.description()),
        }
    }
}

unsafe impl Send for EpitechClientError {}

unsafe impl Sync for EpitechClientError {}
