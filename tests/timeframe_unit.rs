// Unit tests for Timeframe
// Located here to prevent access to private fields
// Enforce testing public interface only, to prevent brittleness

use review_stats::{timeframe::Timeframe, Result};
use std::fmt::Display;

use chrono::{DateTime, TimeDelta, TimeZone, Utc};

#[test]
fn accessors_behave() {
    let start = Utc.timestamp_opt(1_234_567_890, 321).unwrap();
    let finish = Utc.timestamp_opt(9_876_543_210, 123).unwrap();
    let uut = Timeframe::new(start, finish).unwrap();

    assert_eq!(uut.begin(), start);
    assert_eq!(uut.end(), finish);
}

#[test]
fn refuses_reverse_begin_end() {
    let finish = Utc.timestamp_opt(1_234_567_890, 321).unwrap();
    let start = Utc.timestamp_opt(9_876_543_210, 123).unwrap();
    let uut = Timeframe::new(start, finish);

    assert!(uut.is_err_and(|e| e
        .to_string()
        .contains("Timeframes must not end before they begin")));
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
            let uut = Timeframe::new(converter(start), converter(finish)).unwrap();
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
    let uut = Timeframe::from_delta(delta).unwrap();
    let after = Utc::now();

    assert_eq!(uut.end() - uut.begin(), delta);
    within(before, after, uut);
}

#[test]
fn refuses_negative_timedelta() {
    let delta = TimeDelta::days(-123);
    let uut = Timeframe::from_delta(delta);

    assert!(uut.is_err_and(|e| e
        .to_string()
        .contains("Timeframes must not end before they begin")));
}

#[test]
fn constructs_from_string() {
    let nums = [0, 1, 12, 123, 1234];
    let units = ['h', 'd', 'w', 'm'];

    fn calc_delta(num: i64, unit: char) -> TimeDelta {
        match unit {
            'h' => TimeDelta::hours(num),
            'd' => TimeDelta::days(num),
            'w' => TimeDelta::weeks(num),
            'm' => TimeDelta::days(num * Timeframe::MONTH_DAYS),
            _ => panic!("Unknown unit"),
        }
    }

    fn test_case(num: i64, unit: char) {
        let before = Utc::now();
        let uut: Timeframe = num_unit_to_timeframe(num, unit).unwrap();
        let after = Utc::now();

        assert_eq!(uut.end() - uut.begin(), calc_delta(num, unit));
        within(before, after, uut);
    }

    for num in nums {
        for unit in units {
            test_case(num, unit);
        }
    }
}

#[test]
fn refuses_empty_string() {
    let uut: Result<Timeframe> = "".parse();
    assert_invalid_format_error(uut);
}

#[test]
fn refuses_bad_units() {
    let nums = [0, 1, 21, 321, 4321];
    let units = [
        "", " ", "\t", "    ", " \t ", " d ", " up", "+", "=", "x", "9", "asdf", "hdw", "🤪",
    ];

    for num in nums {
        for unit in units {
            assert_invalid_format_error(num_unit_to_timeframe(num, unit));
        }
    }
}

#[test]
fn refuses_bad_numbers() {
    let nums = ["-1", "1.2", "0.1", "0x3ab", "a", ""];
    let units = ["h", "d", "w", "m"];

    for num in nums {
        for unit in units {
            assert_invalid_format_error(num_unit_to_timeframe(num, unit));
        }
    }
}

// Helpers

fn assert_invalid_format_error(result: Result<Timeframe>) {
    assert!(result.is_err_and(|error| error
        .to_string()
        .to_lowercase()
        .contains("invalid timeframe format")));
}

fn num_unit_to_timeframe(num: impl Display, unit: impl Display) -> Result<Timeframe> {
    format!("{}{}", num, unit).parse()
}

fn within(before: DateTime<Utc>, after: DateTime<Utc>, uut: Timeframe) {
    assert!(before <= uut.end() && after >= uut.end());
}
