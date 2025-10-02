use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Timeframe {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
