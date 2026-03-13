#[derive(Debug)]
pub(crate) struct CleanBranchesOptions {
    pub(crate) dry_run: bool,
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}
