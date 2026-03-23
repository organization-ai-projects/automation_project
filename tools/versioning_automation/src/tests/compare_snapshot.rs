//! tools/versioning_automation/src/tests/compare_snapshot.rs

use crate::compare_snapshot;
#[test]
fn fetch_pr_refs_returns_error_for_invalid_pr() {
    let result = compare_snapshot::fetch_pr_refs("__invalid_pr__");
    assert!(result.is_err());
}

#[test]
fn load_compare_snapshot_returns_error_for_invalid_compare_refs() {
    let result = compare_snapshot::load_compare_snapshot("__missing_base__", "__missing_head__");
    assert!(result.is_err());
}
