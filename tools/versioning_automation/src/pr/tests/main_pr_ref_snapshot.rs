//! tools/versioning_automation/src/pr/tests/main_pr_ref_snapshot.rs
use crate::pr::main_pr_ref_snapshot::MainPrRefSnapshot;

#[test]
fn test_fetch_pr_refs_valid() {
    let pr_number = "123";
    let result = MainPrRefSnapshot::fetch_pr_refs(pr_number);
    assert!(result.is_ok());
}

#[test]
fn test_fetch_pr_refs_invalid() {
    let pr_number = "invalid";
    let result = MainPrRefSnapshot::fetch_pr_refs(pr_number);
    assert!(result.is_err());
}
