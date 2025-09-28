use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Timeframe {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl Timeframe {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> DateTime<Utc> {
        self.start
    }
    pub fn end(&self) -> DateTime<Utc> {
        self.end
    }

    pub fn is_inside(&self, timestamp: DateTime<Utc>) -> bool {
        self.start <= timestamp && timestamp <= self.end
    }
}

#[cfg(test)]
mod test {
    use crate::timeframe::Timeframe;
    use chrono::{TimeZone, Utc};

    #[test]
    fn accessors_behave() {
        let begin = Utc.timestamp_opt(1234567890, 321).unwrap();
        let finish = Utc.timestamp_opt(9876543210, 123).unwrap();
        let uut = Timeframe::new(begin, finish);

        assert_eq!(uut.start(), begin);
        assert_eq!(uut.end(), finish);
    }

    #[test]
    fn recognizes_timestamp_inside() {
        let begin = Utc.timestamp_opt(111, 0).unwrap();
        let finish = Utc.timestamp_opt(333, 0).unwrap();
        let inside = Utc.timestamp_opt(222, 0).unwrap();

        let uut = Timeframe::new(begin, finish);
        assert!(uut.is_inside(inside));
    }
}
