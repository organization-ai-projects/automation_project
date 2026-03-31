/// Parsed query terms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Query {
    pub(crate) terms: Vec<String>,
}
