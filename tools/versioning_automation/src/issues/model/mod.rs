//! tools/versioning_automation/src/issues/model/mod.rs
mod close_options;
mod create_options;
mod issue_action;
mod issue_target;
mod read_options;
mod update_options;

#[cfg(test)]
mod tests;

pub(crate) use close_options::CloseOptions;
pub(crate) use create_options::CreateOptions;
pub(crate) use issue_action::IssueAction;
pub(crate) use issue_target::IssueTarget;
pub(crate) use read_options::ReadOptions;
pub(crate) use update_options::UpdateOptions;
