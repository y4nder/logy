use crate::{error::LogyError, log::LogLevel, parser::parse_level};

pub struct CliOptions {
    pub filename: String,
    pub level_filter: Option<LogLevel>,
}

pub fn parse_args(args: &[String]) -> Result<CliOptions, LogyError> {
    let filename = args.get(1).ok_or(LogyError::MissingArgument)?.clone();

    let level_filter = args.iter().find_map(|arg| {
        arg.strip_prefix("--level=")
            .and_then(|lvl| parse_level(lvl).ok())
    });

    Ok(CliOptions {
        filename,
        level_filter,
    })
}
