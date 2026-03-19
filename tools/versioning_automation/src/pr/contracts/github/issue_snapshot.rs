use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct IssueSnapshot {
    #[serde(default)]
    pub(crate) title: String,
    #[serde(default)]
    pub(crate) body: String,
    #[serde(default)]
    pub(crate) labels: Vec<IssueLabel>,
}

use crate::pr::contracts::github::issue_label::IssueLabel;
