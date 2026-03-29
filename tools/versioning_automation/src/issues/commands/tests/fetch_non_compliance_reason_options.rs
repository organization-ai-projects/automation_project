//! tools/versioning_automation/src/issues/commands/tests/fetch_non_compliance_reason_options.rs
use crate::issues::commands::fetch_non_compliance_reason_options::FetchNonComplianceReasonOptions;

#[test]
fn test_run_fetch_non_compliance_reason_success() {
    let options = FetchNonComplianceReasonOptions {
        issue: "123".to_string(),
        repo: Some("test_repo".to_string()),
    };
    let result = options.run_fetch_non_compliance_reason();
    assert_eq!(result, 0);
}

#[test]
fn test_run_fetch_non_compliance_reason_missing_repo() {
    let options = FetchNonComplianceReasonOptions {
        issue: "123".to_string(),
        repo: None,
    };
    let result = options.run_fetch_non_compliance_reason();
    assert_eq!(result, 0);
}
