#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrRefreshValidationOptions {
    pub(crate) pr_number: String,
    pub(crate) repo: Option<String>,
}
