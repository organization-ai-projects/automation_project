//! tools/versioning_automation/src/issues/mod.rs
mod commands;
mod dispatch;
mod execute;
mod issue_comment_payload;
mod issue_comments;
mod parse;
mod render;
mod required_fields;
mod sync_project_status;
pub(crate) mod tasklist_refs;

#[cfg(test)]
mod tests;

pub(crate) use dispatch::run;
