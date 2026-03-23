use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub(crate) struct IssueLabel {
    #[serde(default)]
    pub(crate) name: String,
}
