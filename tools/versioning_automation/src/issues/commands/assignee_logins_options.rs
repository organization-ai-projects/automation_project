//! tools/versioning_automation/src/issues/commands/assignee_logins_options.rs
#[derive(Debug, Clone)]
pub(crate) struct AssigneeLoginsOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}
