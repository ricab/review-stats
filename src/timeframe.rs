use crate::{BoxedError, Result};

use chrono::{DateTime, TimeDelta, Utc};

use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

/// An absolute window of earth time with inclusive boundaries
#[derive(Debug)]
pub struct Timeframe {
    // TODO@ricab enforce invariant end >= begin
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
}

// Custom public interface
impl Timeframe {
    pub const MONTH_DAYS: i64 = 30;

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
        let re = Timeframe::ready_regex();

        let emsg = "Invalid timeframe format (expected '<number><unit>', e.g., '10d', '2w')";
        let captures = re.captures(s).ok_or(emsg)?;
        let (value, unit) = (&captures[1], &captures[2]); // index 0 is the entire match
        let num = value.parse();

        let creator: Result<fn(i64) -> TimeDelta> = match unit {
            "h" => Ok(TimeDelta::hours),
            "d" => Ok(TimeDelta::days),
            "w" => Ok(TimeDelta::weeks),
            "m" => Ok(|unit| TimeDelta::days(unit * Self::MONTH_DAYS)),
            _ => {
                let emsg = format!("Unknown time unit '{}'", unit);
                Err((emsg + " (expected: one of 'h', 'd', 'w', or 'm')").into())
            }
        };

        Ok(Timeframe::from_delta(creator?(num?)))
    }
}

// private helpers
impl Timeframe {
    fn ready_regex() -> &'static Regex {
        static PATTERN: &'static str = r"^(\d+)([hdwm])$";
        static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(PATTERN).unwrap());
        &REGEX
    }
}
