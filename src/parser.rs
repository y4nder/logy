use chrono::NaiveDate;

use crate::{
    error::LogyError,
    log::{LogEntry, LogLevel},
};

pub fn parse_line(line: &str, _strict: bool) -> Result<LogEntry, LogyError> {
    let mut parts = line.splitn(3, ' ');

    let date_str = parts
        .next()
        .ok_or(LogyError::ParseError("missing date".into()))?;

    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| LogyError::ParseError(format!("invalid date: {}", date_str)))?;

    let level_str = parts
        .next()
        .ok_or_else(|| LogyError::ParseError("missing log level".into()))?;
    let message = parts
        .next()
        .ok_or_else(|| LogyError::ParseError("missing message".into()))?
        .to_string();

    let level = parse_level(level_str)?;
    Ok(LogEntry {
        date,
        level,
        message,
    })
}

pub fn parse_level(s: &str) -> Result<LogLevel, LogyError> {
    match s {
        "INFO" => Ok(LogLevel::Info),
        "WARN" => Ok(LogLevel::Warn),
        "ERROR" => Ok(LogLevel::Error),
        _ => Err(LogyError::InvalidLogLevel(s.to_string())),
    }
}
