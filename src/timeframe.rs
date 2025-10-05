use crate::{BoxedError, Result};

use chrono::{DateTime, TimeDelta, Utc};

use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub struct Timeframe {
    // TODO@ricab enforce invariant end >= begin
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
}

/// An absolute window of earth time with inclusive boundaries
impl Timeframe {
    const PATTERN: &'static str = r"^(\d+)([hdwm])$";

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

impl FromStr for Timeframe {
    type Err = BoxedError;
    fn from_str(s: &str) -> Result<Self> {
        const MONTH_DAYS: i64 = 30; // TODO@ricab move to struct level
        let re = Regex::new(Self::PATTERN).unwrap(); // TODO@ricab make this compile time

        let captures = re
            .captures(s)
            .ok_or("Invalid timeframe format (expected '<number><unit>', e.g., '10d', '2w')")?;
        let (value, unit) = (&captures[1], &captures[2]); // index 0 is the entire match
        let num = value.parse();

        let creator: Result<fn(i64) -> TimeDelta> = match unit {
            "h" => Ok(TimeDelta::hours),
            "d" => Ok(TimeDelta::days),
            "w" => Ok(TimeDelta::weeks),
            "m" => Ok(|unit| TimeDelta::days(unit * MONTH_DAYS)),
            _ => {
                let msg = format!("Unknown time unit '{}'", unit);
                Err((msg + " (expected: one of 'h', 'd', 'w', or 'm')").into())
            }
        };

        Ok(Timeframe::from_delta(creator?(num?)))
    }
}
