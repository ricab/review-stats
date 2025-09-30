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
    use chrono::{DateTime, TimeZone, Utc};

    #[test]
    fn accessors_behave() {
        let begin = Utc.timestamp_opt(1234567890, 321).unwrap();
        let finish = Utc.timestamp_opt(9876543210, 123).unwrap();
        let uut = Timeframe::new(begin, finish);

        assert_eq!(uut.start(), begin);
        assert_eq!(uut.end(), finish);
    }

    #[test]
    fn recognizes_timestamp_inside_outside() {
        let cases1 = [
            (111, 333, 222, true),
            (111, 333, 333, true),
            (111, 333, 111, true),
            (111, 333, 112, true),
            (111, 333, 110, false),
            (111, 333, 334, false),
            (111, 222, 333, false),
        ];
        let cases2 = [
            (
                (2025, 9, 20, 23, 59, 59),
                (2025, 9, 30, 0, 0, 0),
                (2025, 9, 30, 12, 0, 0),
                false,
            ),
            (
                (2025, 9, 20, 23, 59, 59),
                (2025, 9, 30, 0, 0, 0),
                (2025, 9, 25, 12, 0, 0),
                true,
            ),
            (
                (2025, 9, 20, 23, 59, 59),
                (2025, 9, 30, 0, 0, 0),
                (2025, 9, 25, 1, 2, 3),
                true,
            ),
        ];

        fn i_to_ts(i: &i64) -> DateTime<Utc> {
            Utc.timestamp_opt(*i, 0).unwrap()
        }

        fn tup_to_ts(tup: &(i32, u32, u32, u32, u32, u32)) -> DateTime<Utc> {
            let (y, m, d, h, min, s) = *tup;
            Utc.with_ymd_and_hms(y, m, d, h, min, s).unwrap()
        }

        fn test_case<T, F>(cases: &[(T, T, T, bool)], converter: F)
        where F: Fn(&T) -> DateTime<Utc> {
            for (begin, finish, other, expect_inside) in cases {
                let uut = Timeframe::new(converter(begin), converter(finish));
                assert_eq!(uut.is_inside(converter(other)), *expect_inside);
            }
        }

        test_case(&cases1, i_to_ts);
        test_case(&cases2, tup_to_ts);
    }
}
