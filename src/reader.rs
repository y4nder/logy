use std::io::BufRead;

use crate::{
    log::{LogEntry, LogLevel},
    parser::parse_line,
};

pub fn extract_log_entries<R: BufRead>(reader: R, level_filter: Option<LogLevel>) -> Vec<LogEntry> {
    reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| parse_line(&line).ok())
        .filter(|e| level_filter.map_or(true, |lvl| e.0 == lvl))
        .collect()
}
