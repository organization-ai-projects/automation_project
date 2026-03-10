#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UnresolvedConflict {
    pub(crate) issue: String,
    pub(crate) reason: String,
}
