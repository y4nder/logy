use core::fmt;
use std::io;

use chrono::NaiveDate;

#[derive(Debug)]
pub enum LogyError {
    InvalidLogLevel(String),
    ParseError(String),
    InvalidRange { since: NaiveDate, until: NaiveDate },
    InvalidDate { field: &'static str, value: String },
    Io(io::Error),
}

impl fmt::Display for LogyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogyError::InvalidLogLevel(level) => write!(f, "invalid log level: {}", level),
            LogyError::Io(err) => write!(f, "I/O error: {}", err),
            LogyError::ParseError(msg) => write!(f, "failed to parse log line: {}", msg),
            LogyError::InvalidRange { since, until } => {
                write!(
                    f,
                    "`since` ({since}) must be earlier than or equal to `until` ({until})"
                )
            }
            LogyError::InvalidDate { field, value, .. } => {
                write!(
                    f,
                    "invalid date for '--{}': '{}', expected format YYYY-MM-DD",
                    field, value
                )
            }
        }
    }
}

impl From<std::io::Error> for LogyError {
    fn from(value: std::io::Error) -> Self {
        LogyError::Io(value)
    }
}

impl From<serde_json::Error> for LogyError {
    fn from(_: serde_json::Error) -> Self {
        LogyError::ParseError("failed to serialize JSON".into())
    }
}

impl std::error::Error for LogyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LogyError::Io(error) => Some(error),
            _ => None,
        }
    }
}
