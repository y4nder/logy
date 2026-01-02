use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Serialize)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
}
