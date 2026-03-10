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

use auto_add::run_auto_add_closes;
use body_context::run_body_context;
use breaking_detect::run_breaking_detect;
use child_pr_refs::run_child_pr_refs;
use closure_marker::run_closure_marker;
use closure_refs::run_closure_refs;
use commands::pr_action::PrAction;
use commands::pr_directives_format::PrDirectivesFormat;
use conflicts::run_directive_conflicts;
use details::run_details;
use directive_conflict_guard::run_directive_conflict_guard;
use directives_apply::run_directives_apply;
use duplicate_actions::run_duplicate_actions;
use group_by_category::run_group_by_category;
use issue_close_policy::run_issue_close_policy;
use issue_context::run_issue_context;
use issue_decision::run_issue_decision;
use issue_ref_kind::run_issue_ref_kind;
use issue_view::run_issue_view;
use non_closing_refs::run_non_closing_refs;
use normalize_issue_key::run_normalize_issue_key;
use open_referencing_issue::run_open_referencing_issue;
use parse::parse;
use pr_state::run_pr_state;
use render::{emit_json, emit_plain, print_usage};
use resolve_category::{
    run_effective_category, run_issue_category_from_labels, run_issue_category_from_title,
    run_resolve_category,
};
use scan::scan_directives;
use sort_bullets::run_sort_bullets;
use state::run_directives_state;
use text_payload::run_text_payload;

pub fn run(args: &[String]) -> i32 {
    match parse(args) {
        Ok(PrAction::Help) => {
            print_usage();
            0
        }
        Ok(PrAction::BreakingDetect(opts)) => run_breaking_detect(opts),
        Ok(PrAction::BodyContext(opts)) => run_body_context(opts),
        Ok(PrAction::ChildPrRefs(opts)) => run_child_pr_refs(opts),
        Ok(PrAction::Directives(opts)) => {
            let records = scan_directives(&opts.text, opts.unique);
            match opts.format {
                PrDirectivesFormat::Plain => {
                    emit_plain(&records);
                    0
                }
                PrDirectivesFormat::Json => emit_json(&records),
            }
        }
        Ok(PrAction::DirectivesApply(opts)) => run_directives_apply(opts),
        Ok(PrAction::Details(opts)) => run_details(opts),
        Ok(PrAction::ClosureRefs(opts)) => run_closure_refs(opts),
        Ok(PrAction::DirectivesState(opts)) => run_directives_state(opts),
        Ok(PrAction::DirectiveConflicts(opts)) => run_directive_conflicts(opts),
        Ok(PrAction::DirectiveConflictGuard(opts)) => run_directive_conflict_guard(opts),
        Ok(PrAction::DuplicateActions(opts)) => run_duplicate_actions(opts),
        Ok(PrAction::EffectiveCategory(opts)) => run_effective_category(opts),
        Ok(PrAction::GroupByCategory(opts)) => run_group_by_category(opts),
        Ok(PrAction::IssueCategoryFromLabels(opts)) => run_issue_category_from_labels(opts),
        Ok(PrAction::IssueCategoryFromTitle(opts)) => run_issue_category_from_title(opts),
        Ok(PrAction::IssueClosePolicy(opts)) => run_issue_close_policy(opts),
        Ok(PrAction::IssueContext(opts)) => run_issue_context(opts),
        Ok(PrAction::IssueView(opts)) => run_issue_view(opts),
        Ok(PrAction::PrState(opts)) => run_pr_state(opts),
        Ok(PrAction::IssueRefKind(opts)) => run_issue_ref_kind(opts),
        Ok(PrAction::NormalizeIssueKey(opts)) => run_normalize_issue_key(opts),
        Ok(PrAction::OpenReferencingIssue(opts)) => run_open_referencing_issue(opts),
        Ok(PrAction::IssueDecision(opts)) => run_issue_decision(opts),
        Ok(PrAction::ClosureMarker(opts)) => run_closure_marker(opts),
        Ok(PrAction::NonClosingRefs(opts)) => run_non_closing_refs(opts),
        Ok(PrAction::ResolveCategory(opts)) => run_resolve_category(opts),
        Ok(PrAction::SortBullets(opts)) => run_sort_bullets(opts),
        Ok(PrAction::AutoAddCloses(opts)) => run_auto_add_closes(opts),
        Ok(PrAction::TextPayload(opts)) => run_text_payload(opts),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}
