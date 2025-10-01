use chrono::{DateTime, TimeDelta, Utc};

#[derive(Debug)]
pub struct Timeframe {
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
}

/// An absolute window of earth time with inclusive boundaries
impl Timeframe {
    pub fn new(begin: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { begin, end }
    }

    pub fn from_delta(delta: TimeDelta) -> Self {
        let now = Utc::now();
        Self {
            begin: now - delta,
            end: now,
        }
    }

    pub fn begin(&self) -> DateTime<Utc> {
        self.begin
    }
    pub fn end(&self) -> DateTime<Utc> {
        self.end
    }

    pub fn is_inside(&self, timestamp: DateTime<Utc>) -> bool {
        self.begin <= timestamp && timestamp <= self.end
    }
}
