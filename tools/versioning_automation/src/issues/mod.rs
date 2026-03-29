//! tools/versioning_automation/src/issues/mod.rs
mod auto_link_relation_snapshot;
mod commands;
mod dispatch;
mod execute;
mod issue_comment_payload;
mod issue_comments;
mod issue_sync_plan;
mod neutralize_ref;
mod neutralize_ref_buckets;
mod parse;
mod render;
mod required_fields;
mod sync_project_status;
mod tasklist_refs;

#[cfg(test)]
mod tests;

pub(crate) use auto_link_relation_snapshot::AutoLinkRelationSnapshot;
pub(crate) use dispatch::run;
pub(crate) use execute::{run_current_login, run_repo_name};
pub(crate) use issue_comment_payload::IssueCommentPayload;
pub(crate) use issue_comments::{find_latest_matching_comment_id, parse_issue_comments};
pub(crate) use issue_sync_plan::IssueSyncPlan;
pub(crate) use neutralize_ref::NeutralizeRef;
pub(crate) use neutralize_ref_buckets::NeutralizeRefBuckets;
pub(crate) use parse::{parse, take_value};
pub(crate) use render::{print_usage, render_direct_issue_body};
pub(crate) use required_fields::{Key, Validation};
pub(crate) use sync_project_status::run_sync_project_status;
pub(crate) use tasklist_refs::extract_tasklist_refs;
