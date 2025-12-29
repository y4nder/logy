use std::collections::HashMap;

use crate::log::{LogEntry, LogLevel};

pub fn count_by_level(entries: &[LogEntry]) -> HashMap<LogLevel, usize> {
    let mut counts = HashMap::new();

    for entry in entries {
        *counts.entry(entry.0).or_insert(0) += 1;
    }

    counts
}
