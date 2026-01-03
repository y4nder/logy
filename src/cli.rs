use chrono::NaiveDate;
use clap::Parser;

use crate::{
    error::{ChronoDateError, LogyError},
    filters::{Filters, SortMode},
    log::LogLevel,
};

#[derive(Parser, Debug)]
#[command(name = "logy", version, about = "A Rust log processing CLI tool")]
pub struct CliArgs {
    /// Path to the log file to process.
    ///
    /// If omitted, Logy reads from standard input (stdin).
    pub filename: Option<String>,

    /// Output logs in structured JSON format.
    ///
    /// Useful for piping into tools like `jq`.
    #[arg(long)]
    pub json: bool,

    /// Enable strict mode.
    ///
    /// When enabled, Logy exits on the first malformed log entry
    /// instead of skipping invalid lines.
    #[arg(long)]
    pub strict: bool,

    /// Stream logs line-by-line instead of processing the entire file.
    ///
    /// Ideal for large files or real-time log monitoring.
    #[arg(long)]
    pub stream: bool,

    /// Sort results in descending order.
    ///
    /// Applies to timestamp-based sorting.
    #[arg(long)]
    pub desc: bool,

    /// Sort log entries by a specific field.
    ///
    /// Possible values depend on the selected sort mode.
    #[arg(long, value_enum, ignore_case = true)]
    pub sort: Option<SortMode>,

    /// Filter logs by log level.
    ///
    /// Only entries matching the specified level are included.
    #[arg(long, value_enum, ignore_case = true)]
    pub level: Option<LogLevel>,

    /// Filter logs starting from this date (inclusive).
    ///
    /// Format: YYYY-MM-DD
    #[arg(long)]
    pub since: Option<String>,

    /// Filter logs until this date (inclusive).
    ///
    /// Format: YYYY-MM-DD
    #[arg(long)]
    pub until: Option<String>,
}

pub struct CliOptions {
    pub filename: Option<String>,
    pub json: bool,
    pub strict: bool,
    pub stream: bool,
    pub filters: Filters,
}

impl TryFrom<CliArgs> for CliOptions {
    type Error = LogyError;

    fn try_from(args: CliArgs) -> Result<Self, Self::Error> {
        let since = match args.since {
            Some(s) => {
                let d = NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|err| {
                    LogyError::InvalidDate {
                        field: "since",
                        value: s.into(),
                        source: Box::new(ChronoDateError(err)),
                    }
                })?;

                Some(d.and_hms_opt(0, 0, 0).unwrap())
            }
            None => None,
        };

        let until = match args.until {
            Some(s) => {
                let d = NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|err| {
                    LogyError::InvalidDate {
                        field: "until",
                        value: s.into(),
                        source: Box::new(ChronoDateError(err)),
                    }
                })?;

                Some(d.and_hms_opt(0, 0, 0).unwrap())
            }
            None => None,
        };

        if let (Some(s), Some(u)) = (since, until) {
            if s > u {
                return Err(LogyError::InvalidRange { since: s, until: u });
            }
        }

        let sort_mode = match args.sort {
            Some(s) => s,
            None => SortMode::Asc,
        };

        Ok(Self {
            filename: args.filename,
            json: args.json,
            strict: args.strict,
            stream: args.stream,
            filters: Filters {
                since: since,
                until: until,
                level: args.level,
                sort: sort_mode,
            },
        })
    }
}
