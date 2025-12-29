use crate::log::{LogEntry, LogLevel};

pub fn parse_line(line: &str) -> Option<LogEntry> {
    let mut parts = line.splitn(3, ' ');

    let _date = parts.next()?;
    let level_str = parts.next()?;
    let message = parts.next()?.to_string();

    let level = match level_str {
        "INFO" => LogLevel::Info,
        "WARN" => LogLevel::Warn,
        "ERROR" => LogLevel::Error,
        _ => return None,
    };

    Some(LogEntry(level, message))
}
