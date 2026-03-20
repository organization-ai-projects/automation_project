#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrDuplicateActionsOptions {
    pub(crate) text: String,
    pub(crate) mode: String,
    pub(crate) repo: String,
    pub(crate) assume_yes: bool,
}
