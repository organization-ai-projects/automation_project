#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrBodyContextOptions {
    pub(crate) pr_number: String,
    pub(crate) repo: Option<String>,
}
