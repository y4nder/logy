use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    log::{LogEntry, LogLevel},
    parser,
};

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

pub fn extract_log_entries(
    reader: BufReader<File>,
    level_filter: Option<LogLevel>,
) -> Vec<LogEntry> {
    reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| parser::parse_line(&line))
        .filter(|e| level_filter.map_or(true, |lvl| e.0 == lvl))
        .collect()
}

pub fn determine_level_filter(args: &Vec<String>) -> Option<LogLevel> {
    args.iter().find_map(|arg| match arg.as_str() {
        "--level=INFO" => Some(LogLevel::Info),
        "--level=WARN" => Some(LogLevel::Warn),
        "--level=ERROR" => Some(LogLevel::Error),
        _ => None,
    })
}
