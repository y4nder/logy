use crate::{error::LogyError, filters::Filters, log::LogEntry, parser::parse_line};
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

        match parse_line(&line, strict) {
            Ok(entry) => {
                if let Some(since_date) = filters.since {
                    if entry.date < since_date {
                        continue;
                    }
                }

                if let Some(until_date) = filters.until {
                    if entry.date > until_date {
                        continue;
                    }
                }

                if filters.level.map_or(true, |lvl| entry.level == lvl) {
                    entries.push(entry);
                }
            }
            Err(e) if strict => return Err(e),
            Err(_) => continue,
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

        match parse_line(&line, strict) {
            Ok(entry) => {
                if let Some(since_date) = filters.since {
                    if entry.date < since_date {
                        continue;
                    }
                }

                if let Some(until_date) = filters.until {
                    if entry.date > until_date {
                        continue;
                    }
                }

                if filters.level.map_or(true, |lvl| entry.level == lvl) {
                    on_entry(entry);
                }
            }
            Err(e) if strict => return Err(e),
            Err(_) => continue,
        }
    }

    Ok(())
}
