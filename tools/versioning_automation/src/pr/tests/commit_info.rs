//! tools/versioning_automation/src/pr/tests/commit_info.rs
use crate::pr::commit_info::CommitInfo;

#[test]
fn test_compare_api_commits_valid() {
    let base_ref = "main";
    let head_ref = "feature";
    let result = CommitInfo::compare_api_commits(base_ref, head_ref);
    assert!(result.is_ok());
}

#[test]
fn test_compare_api_commits_invalid() {
    let base_ref = "invalid";
    let head_ref = "feature";
    let result = CommitInfo::compare_api_commits(base_ref, head_ref);
    assert!(result.is_err());
}
