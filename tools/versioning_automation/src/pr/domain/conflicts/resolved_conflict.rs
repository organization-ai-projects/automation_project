#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedConflict {
    pub(crate) issue: String,
    pub(crate) decision: String,
    pub(crate) origin: String,
}
