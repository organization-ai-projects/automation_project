#[derive(Debug, Clone)]
pub(crate) struct FetchNonComplianceReasonOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}
