//! projects/products/stable/code_agent_sandbox/backend/src/sandbox_engine/tests/records.rs
use crate::journal::Journal;
use crate::sandbox_engine::records::check_file_limit;

#[test]
fn check_file_limit_pushes_policy_violation_when_exceeded() {
    let file = tempfile::NamedTempFile::new().expect("journal file");
    let mut journal = Journal::new(file.path().to_path_buf()).expect("journal");
    let mut results = Vec::new();

    let exceeded =
        check_file_limit(5, 3, "run-1", &mut journal, &mut results).expect("check should succeed");

    assert!(exceeded);
    assert_eq!(results.len(), 1);
    assert!(!results[0].ok);
    assert_eq!(results[0].kind, "PolicyViolation");
}
