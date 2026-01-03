use core::fmt;
use std::collections::HashMap;

use chrono::NaiveDateTime;
use clap::ValueEnum;
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
    pub timestamp: Option<NaiveDateTime>,
    pub level: Option<LogLevel>,
    pub message: String,

    pub fields: HashMap<String, String>,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ts = self
            .timestamp
            .map(|t| t.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "-".into());

        let level = self
            .level
            .map(|l| format!("{:?}", l).to_uppercase())
            .unwrap_or_else(|| "-".into());

        write!(f, "{} {:<5} {}", ts, level, self.message)
    }
}
