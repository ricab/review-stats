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

    pub fn start(&self) -> DateTime<Utc> { self.start }
    pub fn end(&self) -> DateTime<Utc> { self.end }
}

#[cfg(test)]
mod test {
    use chrono::{TimeZone, Utc};
    use crate::timeframe::Timeframe;

    #[test]
    fn accessors_behave() {
        let begin = Utc.timestamp_opt(1234567890, 321).unwrap();
        let finish = Utc.timestamp_opt(9876543210, 123).unwrap();
        let uut = Timeframe::new(begin, finish);

        assert_eq!(uut.start(), begin);
        assert_eq!(uut.end(), finish);
    }
}
