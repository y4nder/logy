use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
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
