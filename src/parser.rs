use std::collections::HashMap;

use chrono::NaiveDate;

use crate::{
    error::{LogyError, ParseMsg},
    log::{LogEntry, LogLevel},
};

pub(crate) fn parse_line(line: &str) -> Result<LogEntry, LogyError> {
    let mut parts = line.splitn(3, ' ');

    let date_str = parts.next();
    let timestamp = date_str
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .and_then(|d| d.and_hms_opt(0, 0, 0));

    let level_str = parts.next();
    let message = parts
        .next()
        .ok_or_else(|| LogyError::ParseError {
            source: Box::new(ParseMsg("missing message")),
        })?
        .to_string();

    let level = level_str.and_then(|s| parse_level(s).ok());

    Ok(LogEntry {
        timestamp,
        level: level,
        message,
        fields: HashMap::new(),
    })
}

fn parse_level(s: &str) -> Result<LogLevel, ()> {
    match s {
        "INFO" => Ok(LogLevel::Info),
        "WARN" => Ok(LogLevel::Warn),
        "ERROR" => Ok(LogLevel::Error),
        _ => Err(()),
    }
}
