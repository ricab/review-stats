// Unit tests for RepoInstance. See README.md about the organization.

use review_stats::{repo_instance::RepoInstance, Result};

#[test]
fn parses_valid_owner_repo() {
    let uut: RepoInstance = "owner/repo".parse().unwrap();
    assert_eq!(uut.owner, "owner");
    assert_eq!(uut.repo, "repo");
}

#[test]
fn parses_owner_repo_with_hyphens() {
    let uut: RepoInstance = "my-owner/my-repo".parse().unwrap();
    assert_eq!(uut.owner, "my-owner");
    assert_eq!(uut.repo, "my-repo");
}

#[test]
fn parses_owner_repo_with_underscores() {
    let uut: RepoInstance = "my_owner/my_repo".parse().unwrap();
    assert_eq!(uut.owner, "my_owner");
    assert_eq!(uut.repo, "my_repo");
}

#[test]
fn parses_repo_with_dots() {
    let uut: RepoInstance = "owner/my.repo.name".parse().unwrap();
    assert_eq!(uut.owner, "owner");
    assert_eq!(uut.repo, "my.repo.name");
}

#[test]
fn parses_numeric_names() {
    let uut: RepoInstance = "123/456".parse().unwrap();
    assert_eq!(uut.owner, "123");
    assert_eq!(uut.repo, "456");
}

#[test]
fn parses_mixed_case() {
    let uut: RepoInstance = "OwnerName/RepoName".parse().unwrap();
    assert_eq!(uut.owner, "OwnerName");
    assert_eq!(uut.repo, "RepoName");
}

#[test]
fn refuses_missing_slash() {
    let uut: Result<RepoInstance> = "ownerrepo".parse();
    assert_invalid_format(uut);
}

#[test]
fn refuses_empty_owner() {
    let uut: Result<RepoInstance> = "/repo".parse();
    assert_invalid_format(uut);
}

#[test]
fn refuses_empty_repo() {
    let uut: Result<RepoInstance> = "owner/".parse();
    assert_invalid_format(uut);
}

#[test]
fn refuses_empty_string() {
    let uut: Result<RepoInstance> = "".parse();
    assert_invalid_format(uut);
}

#[test]
fn refuses_multiple_slashes() {
    let uut: Result<RepoInstance> = "owner/repo/extra".parse();
    assert_invalid_format(uut);
}

#[test]
fn refuses_query_string_injection() {
    let uut: Result<RepoInstance> = "owner/repo?admin=true".parse();
    assert_invalid_format(uut);
}

#[test]
fn refuses_fragment_injection() {
    let uut: Result<RepoInstance> = "owner/repo#fragment".parse();
    assert_invalid_format(uut);
}

#[test]
fn refuses_path_traversal() {
    let cases = ["../owner/repo", "owner/../repo", "owner/repo/.."];
    for case in cases {
        let uut: Result<RepoInstance> = case.parse();
        assert_invalid_format(uut);
    }
}

#[test]
fn refuses_owner_exceeding_max_length() {
    let long_owner = "a".repeat(40);
    let input = format!("{}/repo", long_owner);
    let uut: Result<RepoInstance> = input.parse();
    assert_invalid_format(uut);
}

#[test]
fn refuses_repo_exceeding_max_length() {
    let long_repo = "a".repeat(40);
    let input = format!("owner/{}", long_repo);
    let uut: Result<RepoInstance> = input.parse();
    assert_invalid_format(uut);
}

#[test]
fn accepts_max_length_names() {
    let max_owner = "a".repeat(39);
    let max_repo = "b".repeat(39);
    let input = format!("{}/{}", max_owner, max_repo);
    let uut: RepoInstance = input.parse().unwrap();
    assert_eq!(uut.owner, max_owner);
    assert_eq!(uut.repo, max_repo);
}

#[test]
fn displays_as_owner_slash_repo() {
    let uut = RepoInstance::new("myowner".to_string(), "myrepo".to_string());
    assert_eq!(uut.to_string(), "myowner/myrepo");
}

// Helpers

fn assert_invalid_format(result: Result<RepoInstance>) {
    assert!(result.is_err_and(|e| e.to_string().contains("Invalid repository format")));
}
