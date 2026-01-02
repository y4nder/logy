use core::fmt;
use std::io;

#[derive(Debug)]
pub enum LogyError {
    MissingArgument,
    InvalidLogLevel(String),
    ParseError(&'static str),
    Io(io::Error),
}

impl fmt::Display for LogyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogyError::MissingArgument => write!(f, "missing required command-line arguments"),
            LogyError::InvalidLogLevel(level) => write!(f, "invalid log level: {}", level),
            LogyError::Io(err) => write!(f, "I/O error: {}", err),
            LogyError::ParseError(msg) => write!(f, "failed to parse log line: {}", msg),
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
        LogyError::ParseError("failed to serialize JSON")
    }
}
