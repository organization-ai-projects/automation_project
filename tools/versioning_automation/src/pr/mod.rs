mod auto_add;
mod breaking_detect;
mod child_pr_refs;
mod closure_marker;
mod commands;
mod commit_info;
mod contracts;
mod directive_conflict_guard;
mod domain;
mod duplicate_actions;
mod execute;
mod generate_description;
mod generate_options;
mod group_by_category;
mod issue_close_policy;
mod issue_context;
mod issue_decision;
mod issue_outcomes_snapshot;
mod issue_ref_kind;
mod main_pr_ref_snapshot;
mod normalize_issue_key;
mod parse;
mod pr_state;
mod refresh_validation;
mod render;
mod resolve_category;
mod sort_bullets;
mod state;
mod text_payload;
mod update_body;

#[cfg(test)]
mod tests;

pub(crate) use breaking_detect::text_indicates_breaking;
pub(crate) use commands::PrDirectiveConflictGuardOptions;
pub(crate) use commands::{PrDirectiveConflictsOptions, PrIssueContextOptions};
pub(crate) use commit_info::CommitInfo;
pub(crate) use contracts::github::issue_label::IssueLabel;
pub(crate) use domain::directives::DirectiveRecord;
pub(crate) use domain::directives::DirectiveRecordType;
pub(crate) use execute::run;
pub(crate) use issue_context::load_issue_context_payload;
pub(crate) use issue_outcomes_snapshot::IssueOutcomesSnapshot;
pub(crate) use main_pr_ref_snapshot::MainPrRefSnapshot;
pub(crate) use resolve_category::{issue_category_from_labels, resolve_effective_category};
pub(crate) use state::State;
pub(crate) use text_payload::{
    extract_effective_action_issue_numbers, extract_effective_issue_ref_records,
    extract_effective_issue_ref_sets, load_pr_text_payload,
};
