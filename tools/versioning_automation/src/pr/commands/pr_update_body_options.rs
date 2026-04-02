#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrUpdateBodyOptions {
    pub(crate) pr_number: String,
    pub(crate) repo: Option<String>,
    pub(crate) body: String,
}
