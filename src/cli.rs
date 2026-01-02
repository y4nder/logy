use chrono::NaiveDate;
use clap::Parser;

use crate::{
    error::LogyError,
    filters::{Filters, SortMode},
    log::LogLevel,
};

#[derive(Parser, Debug)]
#[command(name = "logy", version, about = "A Rust log processing CLI tool")]
pub struct CliArgs {
    pub filename: Option<String>,

    #[arg(long)]
    pub json: bool,

    #[arg(long)]
    pub strict: bool,

    #[arg(long)]
    pub stream: bool,

    #[arg(long)]
    pub desc: bool,

    #[arg(long, value_enum, ignore_case = true)]
    pub sort: Option<SortMode>,

    #[arg(long, value_enum, ignore_case = true)]
    pub level: Option<LogLevel>,

    /// Filter logs starting from this date (YYYY-MM-DD)
    #[arg(long)]
    pub since: Option<String>,

    /// Filter logs until this date (YYYY-MM-DD)
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
            Some(s) => Some(NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|_| {
                LogyError::InvalidDate {
                    field: "since",
                    value: s.into(),
                }
            })?),
            None => None,
        };

        let until = match args.until {
            Some(s) => Some(NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|_| {
                LogyError::InvalidDate {
                    field: "until",
                    value: s.into(),
                }
            })?),
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
