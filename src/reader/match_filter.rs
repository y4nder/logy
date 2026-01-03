use crate::{LogEntry, filters::Filters};

pub fn matches_filter(entry: &LogEntry, filters: &Filters) -> bool {
    // since filter
    if let Some(since) = filters.since {
        match entry.timestamp {
            Some(ts) if ts >= since => {}
            _ => return false,
        }
    }

    // until filter
    if let Some(until) = filters.until {
        match entry.timestamp {
            Some(ts) if ts <= until => {}
            _ => return false,
        }
    }

    // level filter
    if let Some(filter_level) = filters.level {
        if entry.level != Some(filter_level) {
            return false;
        }
    }

    true
}
