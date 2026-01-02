use chrono::NaiveDate;

use crate::{error::LogyError, filters::Filters, parser::parse_level};

pub struct CliOptions {
    pub filename: Option<String>,
    pub json: bool,
    pub strict: bool,
    pub stream: bool,
    pub filters: Filters,
    pub desc: bool,
}

pub fn parse_args(args: &[String]) -> Result<CliOptions, LogyError> {
    let filename = args
        .iter()
        .skip(1)
        .find(|arg| !arg.starts_with("--"))
        .cloned();

    let level_filter = args.iter().find_map(|arg| {
        arg.strip_prefix("--level=")
            .and_then(|lvl| parse_level(lvl).ok())
    });

    let since = args.iter().find_map(|arg| {
        arg.strip_prefix("--since=")
            .and_then(|date_str| NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok())
    });

    let until = args.iter().find_map(|arg| {
        arg.strip_prefix("--until=")
            .and_then(|date_str| NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok())
    });

    if let (Some(s), Some(u)) = (since, until) {
        if s > u {
            return Err(LogyError::InvalidRange { since: s, until: u });
        }
    }

    let json = args.iter().any(|arg| arg == "--json");
    let strict = args.iter().any(|arg| arg == "--strict");
    let stream = args.iter().any(|arg| arg == "--stream");
    let desc = args.iter().any(|arg| arg == "--desc");

    Ok(CliOptions {
        filename,
        json,
        strict,
        stream,
        filters: Filters {
            since,
            until,
            level: level_filter,
        },
        desc,
    })
}
