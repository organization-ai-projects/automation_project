//! tools/versioning_automation/src/issues/required_fields/gh_issue_label.rs
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct GhIssueLabel {
    pub(crate) name: Option<String>,
}
