use crate::{
    error::LogyError, filters::Filters, log::LogEntry, parser::parse_line,
    reader::match_filter::matches_filter,
};
use std::io::BufRead;

pub fn extract_log_entries<R: BufRead>(
    reader: R,
    filters: &Filters,
    strict: bool,
) -> Result<Vec<LogEntry>, LogyError> {
    let mut entries = Vec::new();
    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        }

        let entry = match parse_line(&line) {
            Ok(e) => e,
            Err(e) if strict => return Err(e),
            Err(_) => continue,
        };

        if matches_filter(&entry, &filters) {
            entries.push(entry);
        }
    }
    Ok(entries)
}

pub fn stream_logs<R: BufRead>(
    reader: R,
    filters: &Filters,
    strict: bool,
    mut on_entry: impl FnMut(LogEntry),
) -> Result<(), LogyError> {
    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        }

        let entry = match parse_line(&line) {
            Ok(e) => e,
            Err(e) if strict => return Err(e),
            Err(_) => continue,
        };

        if matches_filter(&entry, &filters) {
            on_entry(entry);
        }
    }

    Ok(())
}
