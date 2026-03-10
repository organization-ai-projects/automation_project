use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct IssueLabel {
    #[serde(default)]
    pub(crate) name: String,
}
