use crate::{error::LogyError, log::LogLevel, parser::parse_level};

pub struct CliOptions {
    pub filename: Option<String>,
    pub level_filter: Option<LogLevel>,
    pub json: bool,
    pub strict: bool,
    pub stream: bool,
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

    let json = args.iter().any(|arg| arg == "--json");
    let strict = args.iter().any(|arg| arg == "--strict");
    let stream = args.iter().any(|arg| arg == "--stream");

    Ok(CliOptions {
        filename,
        level_filter,
        json,
        strict,
        stream,
    })
}
