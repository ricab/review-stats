use crate::{BoxedError, Result};

use chrono::{DateTime, TimeDelta, Utc};

use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

/// An absolute window of forward earth time with inclusive boundaries
#[derive(Debug)]
pub struct Timeframe {
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
}

// Custom public interface
impl Timeframe {
    pub const MONTH_DAYS: i64 = 30;

    pub fn new(begin: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self> {
        if end < begin {
            Err("Timeframes must not end before they begin".into())
        } else {
            Ok(Self { begin, end })
        }
    }

    pub fn from_delta(delta: TimeDelta) -> Result<Self> {
        if delta < TimeDelta::zero() {
            Err("Timeframes must not end before they begin".into())
        } else {
            let now = Utc::now();
            Ok(Self {
                begin: now - delta,
                end: now,
            })
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

        let creator: Result<fn(i64) -> TimeDelta> = match unit {
            "h" => Ok(TimeDelta::hours),
            "d" => Ok(TimeDelta::days),
            "w" => Ok(TimeDelta::weeks),
            "m" => Ok(|unit| TimeDelta::days(unit * Self::MONTH_DAYS)),
            _ => unreachable!("Unknown unit '{}' shouldn't match regex", unit),
        };

        let num = value.parse()?;
        let delta = creator?(num);
        if delta < TimeDelta::zero() {
            unreachable!("Negative value '{}' shouldn't match regex", num);
        }

        Ok(Timeframe::from_delta(delta).expect("The delta should be valid at this point"))
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
