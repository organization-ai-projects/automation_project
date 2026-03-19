#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrDirectiveConflictsOptions {
    pub(crate) text: String,
    pub(crate) source_branch_count: u32,
}
