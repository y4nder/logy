use chrono::NaiveDate;

use crate::log::LogLevel;

#[derive(Debug, Clone)]
pub struct Filters {
    pub level: Option<LogLevel>,
    pub since: Option<NaiveDate>,
    pub until: Option<NaiveDate>,
}
