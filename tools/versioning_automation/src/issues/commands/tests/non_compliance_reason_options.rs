//! tools/versioning_automation/src/issues/commands/tests/non_compliance_reason_options.rs
use crate::issues::commands::non_compliance_reason_options::NonComplianceReasonOptions;

#[test]
fn test_run_non_compliance_reason() {
    let options = NonComplianceReasonOptions {
        title: "Test Title".to_string(),
        body: "Test Body".to_string(),
        labels_raw: "bug".to_string(),
    };
    let result = options.run_non_compliance_reason();
    assert_eq!(result, 0);
}
