use core::fmt;
use std::io;

use chrono::NaiveDate;

#[derive(Debug)]
pub enum LogyError {
    InvalidLogLevel(String),
    ParseError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    InvalidRange {
        since: NaiveDate,
        until: NaiveDate,
    },
    InvalidDate {
        field: &'static str,
        value: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    Io(io::Error),
}

impl fmt::Display for LogyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogyError::InvalidLogLevel(level) => write!(f, "invalid log level: {}", level),
            LogyError::Io(err) => write!(f, "I/O error: {}", err),
            LogyError::ParseError { .. } => write!(f, "failed to parse log line"),
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
    fn from(err: serde_json::Error) -> Self {
        LogyError::ParseError {
            source: Box::new(err),
        }
    }
}

impl std::error::Error for LogyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LogyError::InvalidDate { source, .. } => Some(source.as_ref()),
            LogyError::ParseError { source } => Some(source.as_ref()),
            LogyError::Io(error) => Some(error),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ParseMsg(pub &'static str);

impl std::fmt::Display for ParseMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseMsg {}

#[derive(Debug)]
pub struct ChronoDateError(pub chrono::ParseError);

impl std::fmt::Display for ChronoDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for ChronoDateError {}
