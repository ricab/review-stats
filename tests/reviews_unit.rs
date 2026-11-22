// Unit tests for reviews. See README.md about the organization.

use review_stats::reviews::Stats;

fn stats_with(counts: &[(&str, u32)]) -> Stats {
    Stats {
        review_counts: counts.iter().map(|&(r, c)| (r.to_string(), c)).collect(),
    }
}

#[test]
fn sorts_counts_descending() {
    let uut = stats_with(&[("alice", 1), ("bob", 3), ("carol", 2)]);
    assert_eq!(uut.sorted_counts(), [("bob", 3), ("carol", 2), ("alice", 1)]);
}

#[test]
fn breaks_count_ties_by_name() {
    let uut = stats_with(&[("carol", 2), ("alice", 2), ("bob", 2)]);
    assert_eq!(uut.sorted_counts(), [("alice", 2), ("bob", 2), ("carol", 2)]);
}

#[test]
fn sorts_no_counts() {
    let uut = stats_with(&[]);
    assert!(uut.sorted_counts().is_empty());
}
