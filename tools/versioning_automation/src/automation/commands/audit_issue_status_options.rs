#[derive(Debug, Clone)]
pub(crate) struct AuditIssueStatusOptions {
    pub(crate) repo: Option<String>,
    pub(crate) base_ref: String,
    pub(crate) head_ref: String,
    pub(crate) limit: usize,
    pub(crate) output_file: Option<String>,
}
