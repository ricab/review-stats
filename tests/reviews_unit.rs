// Unit tests for reviews. See README.md about the organization.

use review_stats::reviews::{Ledger, ReviewRecord};

fn ledger_with(counts: &[(&str, usize)]) -> Ledger {
    Ledger {
        reviews: counts.iter()
            .map(|&(r, c)| (r.to_string(), (0..c).map(|_| ReviewRecord).collect()))
            .collect(),
    }
}

#[test]
fn sorts_counts_descending() {
    let uut = ledger_with(&[("alice", 1), ("bob", 3), ("carol", 2)]);
    assert_eq!(uut.sorted_counts(), [("bob", 3), ("carol", 2), ("alice", 1)]);
}

#[test]
fn breaks_count_ties_by_name() {
    let uut = ledger_with(&[("carol", 2), ("alice", 2), ("bob", 2)]);
    assert_eq!(uut.sorted_counts(), [("alice", 2), ("bob", 2), ("carol", 2)]);
}

#[test]
fn breaks_count_ties_ignoring_case() {
    let uut = ledger_with(&[("bob", 2), ("Carol", 2), ("Alice", 2)]);
    assert_eq!(uut.sorted_counts(), [("Alice", 2), ("bob", 2), ("Carol", 2)]);
}

#[test]
fn sorts_no_counts() {
    let uut = ledger_with(&[]);
    assert!(uut.sorted_counts().is_empty());
}
