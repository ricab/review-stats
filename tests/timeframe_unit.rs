// Unit tests for Timeframe
// Located here to prevent access to private fields
// Enforce testing public interface only, to prevent brittleness

use review_stats::timeframe::Timeframe;

use chrono::{DateTime, TimeDelta, TimeZone, Utc};

#[test]
fn accessors_behave() {
    let start = Utc.timestamp_opt(1234567890, 321).unwrap();
    let finish = Utc.timestamp_opt(9876543210, 123).unwrap();
    let uut = Timeframe::new(start, finish);

    assert_eq!(uut.begin(), start);
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
    where
        F: Fn(&T) -> DateTime<Utc>,
    {
        for (start, finish, other, expect_inside) in cases {
            let uut = Timeframe::new(converter(start), converter(finish));
            assert_eq!(uut.is_inside(converter(other)), *expect_inside);
        }
    }

    test_case(&cases1, i_to_ts);
    test_case(&cases2, tup_to_ts);
}

#[test]
fn constructs_from_timedelta() {
    let delta = TimeDelta::days(123);

    let before = Utc::now();
    let uut = Timeframe::from_delta(delta);
    let after = Utc::now();

    assert_eq!(uut.end() - uut.begin(), delta);
    assert!(before <= uut.end() && after >= uut.end());
}

#[test]
fn constructs_from_days_string() {
    let days = 182; // TODO@ricab add consts?
    let expect_delta = TimeDelta::days(days);

    let before = Utc::now();
    let uut: Timeframe = format!("{}d", days).parse().unwrap();
    let after = Utc::now();

    assert_eq!(uut.end() - uut.begin(), expect_delta);
    assert!(before <= uut.end() && after >= uut.end()); // TODO@ricab extract this
}
