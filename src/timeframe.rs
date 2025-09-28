use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Timeframe {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
