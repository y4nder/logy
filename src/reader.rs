use std::io::BufRead;

use crate::{
    error::LogyError,
    log::{LogEntry, LogLevel},
    parser::parse_line,
};

pub fn extract_log_entries<R: BufRead>(
    reader: R,
    level_filter: Option<LogLevel>,
    strict: bool,
) -> Result<Vec<LogEntry>, LogyError> {
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;

        match parse_line(&line) {
            Ok(entry) => {
                if level_filter.map_or(true, |lvl| entry.level == lvl) {
                    entries.push(entry);
                }
            }
            Err(e) if strict => return Err(e),
            Err(_) => continue,
        }
    }

    Ok(entries)
}
