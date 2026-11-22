// Unit tests for reviews. See README.md about the organization.

use review_stats::reviews::{Ledger, Record};

fn dummy_record() -> Record {
    Record { url: String::new() }
}

fn ledger_with(counts: &[(&str, usize)]) -> Ledger {
    Ledger {
        reviews: counts.iter()
            .map(|&(r, c)| (r.to_string(), (0..c).map(|_| dummy_record()).collect()))
            .collect(),
    }
}

#[test]
fn sorts_by_count_descending() {
    let uut = ledger_with(&[("alice", 1), ("bob", 3), ("carol", 2)]);
    let order: Vec<_> = uut.sorted().iter().map(|&(reviewer, _)| reviewer).collect();
    assert_eq!(order, ["bob", "carol", "alice"]);
}

#[test]
fn breaks_count_ties_by_name() {
    let uut = ledger_with(&[("carol", 2), ("alice", 2), ("bob", 2)]);
    let order: Vec<_> = uut.sorted().iter().map(|&(reviewer, _)| reviewer).collect();
    assert_eq!(order, ["alice", "bob", "carol"]);
}

#[test]
fn breaks_count_ties_ignoring_case() {
    let uut = ledger_with(&[("bob", 2), ("Carol", 2), ("Alice", 2)]);
    let order: Vec<_> = uut.sorted().iter().map(|&(reviewer, _)| reviewer).collect();
    assert_eq!(order, ["Alice", "bob", "Carol"]);
}

#[test]
fn sorts_empty_ledger() {
    let uut = ledger_with(&[]);
    assert!(uut.sorted().is_empty());
}

#[test]
fn keeps_records_with_their_reviewer() {
    let record = |url: &str| Record { url: url.to_string() };
    let uut = Ledger {
        reviews: [
            ("alice".to_string(), vec![record("a1")]),
            ("bob".to_string(), vec![record("b1"), record("b2")]),
        ].into(),
    };

    let urls: Vec<(&str, Vec<&str>)> = uut.sorted().into_iter()
        .map(|(reviewer, records)| (reviewer, records.iter().map(|r| r.url.as_str()).collect()))
        .collect();
    assert_eq!(urls, [("bob", vec!["b1", "b2"]), ("alice", vec!["a1"])]);
}
