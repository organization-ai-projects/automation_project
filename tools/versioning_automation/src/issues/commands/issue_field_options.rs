//! tools/versioning_automation/src/issues/commands/issue_field_options.rs
use crate::issues::commands::IssueFieldName;

#[derive(Debug, Clone)]
pub(crate) struct IssueFieldOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
    pub(crate) name: IssueFieldName,
}
