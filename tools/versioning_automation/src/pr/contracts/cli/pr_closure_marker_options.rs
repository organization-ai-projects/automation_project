#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrClosureMarkerOptions {
    pub(crate) text: String,
    pub(crate) keyword_pattern: String,
    pub(crate) issue: String,
    pub(crate) mode: String,
}
