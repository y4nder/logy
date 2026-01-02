use clap::ValueEnum;
use chrono::NaiveDate;
use serde::Serialize;


#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}


#[derive(Debug, Serialize)]
pub struct LogEntry {
    pub date: NaiveDate,
    pub level: LogLevel,
    pub message: String,
}
