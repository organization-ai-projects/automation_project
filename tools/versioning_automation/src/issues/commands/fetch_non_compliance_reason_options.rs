//! tools/versioning_automation/src/issues/commands/fetch_non_compliance_reason_options.rs

#[derive(Debug, Clone)]
pub(crate) struct FetchNonComplianceReasonOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}
