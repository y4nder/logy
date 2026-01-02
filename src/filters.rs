use chrono::NaiveDate;
use clap::ValueEnum;
use serde::Serialize;

use crate::log::LogLevel;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum SortMode {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct Filters {
    pub level: Option<LogLevel>,
    pub since: Option<NaiveDate>,
    pub until: Option<NaiveDate>,
    pub sort: SortMode,
}
