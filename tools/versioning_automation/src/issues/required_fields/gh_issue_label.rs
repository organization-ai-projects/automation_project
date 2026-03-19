use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct GhIssueLabel {
    pub(crate) name: Option<String>,
}
