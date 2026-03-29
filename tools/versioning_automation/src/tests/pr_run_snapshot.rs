//! tools/versioning_automation/src/tests/pr_run_snapshot.rs
use crate::pr_run_snapshot::PrRunSnapshot;

#[test]
fn load_pr_run_snapshot_returns_error_for_invalid_compare_refs() {
    let result = PrRunSnapshot::load_pr_run_snapshot("__missing_base__", "__missing_head__");
    assert!(result.is_err());
}
