use chrono::NaiveDate;

use crate::{
    error::{ChronoDateError, LogyError, ParseMsg},
    log::{LogEntry, LogLevel},
};

pub(crate) fn parse_line(line: &str, _strict: bool) -> Result<LogEntry, LogyError> {
    let mut parts = line.splitn(3, ' ');

    let date_str = parts.next().ok_or(LogyError::ParseError {
        source: Box::new(ParseMsg("missing date")),
    })?;

    let date =
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|err| LogyError::InvalidDate {
            field: "date",
            value: date_str.into(),
            source: Box::new(ChronoDateError(err)),
        })?;

    let level_str = parts.next().ok_or_else(|| LogyError::ParseError {
        source: Box::new(ParseMsg("missing log level")),
    })?;
    let message = parts
        .next()
        .ok_or_else(|| LogyError::ParseError {
            source: Box::new(ParseMsg("missing message")),
        })?
        .to_string();

    let level = parse_level(level_str)?;
    Ok(LogEntry {
        date,
        level,
        message,
    })
}

fn parse_level(s: &str) -> Result<LogLevel, LogyError> {
    match s {
        "INFO" => Ok(LogLevel::Info),
        "WARN" => Ok(LogLevel::Warn),
        "ERROR" => Ok(LogLevel::Error),
        _ => Err(LogyError::InvalidLogLevel(s.to_string())),
    }
}
