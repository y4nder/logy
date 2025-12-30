use crate::{
    error::LogyError,
    log::{LogEntry, LogLevel},
};

pub fn parse_line(line: &str) -> Result<LogEntry, LogyError> {
    let mut parts = line.splitn(3, ' ');

    let _date = parts
        .next()
        .ok_or_else(|| LogyError::ParseError("missing date"))?;
    let level_str = parts
        .next()
        .ok_or_else(|| LogyError::ParseError("missing log level"))?;
    let message = parts
        .next()
        .ok_or_else(|| LogyError::ParseError("missing message"))?
        .to_string();

    let level = parse_level(level_str)?;
    Ok(LogEntry(level, message))
}

pub fn parse_level(s: &str) -> Result<LogLevel, LogyError> {
    match s {
        "INFO" => Ok(LogLevel::Info),
        "WARN" => Ok(LogLevel::Warn),
        "ERROR" => Ok(LogLevel::Error),
        _ => Err(LogyError::InvalidLogLevel(s.to_string())),
    }
}
