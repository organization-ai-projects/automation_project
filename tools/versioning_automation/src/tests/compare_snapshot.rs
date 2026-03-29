//! tools/versioning_automation/src/tests/compare_snapshot.rs
use crate::compare_snapshot::CompareSnapshot;

#[test]
fn load_compare_snapshot_returns_error_for_invalid_compare_refs() {
    let result = CompareSnapshot::load_compare_snapshot("__missing_base__", "__missing_head__");
    assert!(result.is_err());
}
