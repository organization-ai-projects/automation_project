mod auto_add;
mod body_context;
mod breaking_detect;
mod child_pr_refs;
mod closure_marker;
mod closure_refs;
mod commands;
mod conflicts;
mod contracts;
mod details;
mod directive_conflict_guard;
mod directives_apply;
mod domain;
mod duplicate_actions;
mod execute;
mod group_by_category;
mod issue_close_policy;
mod issue_context;
mod issue_decision;
mod issue_ref_kind;
mod issue_view;
mod non_closing_refs;
mod normalize_issue_key;
mod open_referencing_issue;
mod parse;
mod pr_state;
mod render;
mod resolve_category;
mod scan;
mod sort_bullets;
mod state;
mod text_payload;

#[cfg(test)]
mod tests;

pub fn run(args: &[String]) -> i32 {
    execute::run(args)
}
