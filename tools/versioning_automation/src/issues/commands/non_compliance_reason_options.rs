//! tools/versioning_automation/src/issues/commands/non_compliance_reason_options.rs
#[derive(Debug, Clone)]
pub(crate) struct NonComplianceReasonOptions {
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) labels_raw: String,
}
