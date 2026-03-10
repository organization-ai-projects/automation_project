use crate::pr::auto_add::run_auto_add_closes;
use crate::pr::body_context::run_body_context;
use crate::pr::breaking_detect::run_breaking_detect;
use crate::pr::child_pr_refs::run_child_pr_refs;
use crate::pr::closure_marker::run_closure_marker;
use crate::pr::closure_refs::run_closure_refs;
use crate::pr::commands::pr_action::PrAction;
use crate::pr::commands::pr_directives_format::PrDirectivesFormat;
use crate::pr::conflicts::run_directive_conflicts;
use crate::pr::details::run_details;
use crate::pr::directive_conflict_guard::run_directive_conflict_guard;
use crate::pr::directives_apply::run_directives_apply;
use crate::pr::duplicate_actions::run_duplicate_actions;
use crate::pr::group_by_category::run_group_by_category;
use crate::pr::issue_close_policy::run_issue_close_policy;
use crate::pr::issue_context::run_issue_context;
use crate::pr::issue_decision::run_issue_decision;
use crate::pr::issue_ref_kind::run_issue_ref_kind;
use crate::pr::issue_view::run_issue_view;
use crate::pr::non_closing_refs::run_non_closing_refs;
use crate::pr::normalize_issue_key::run_normalize_issue_key;
use crate::pr::open_referencing_issue::run_open_referencing_issue;
use crate::pr::parse::parse;
use crate::pr::pr_state::run_pr_state;
use crate::pr::render::{emit_json, emit_plain, print_usage};
use crate::pr::resolve_category::{
    run_effective_category, run_issue_category_from_labels, run_issue_category_from_title,
    run_resolve_category,
};
use crate::pr::scan::scan_directives;
use crate::pr::sort_bullets::run_sort_bullets;
use crate::pr::state::run_directives_state;
use crate::pr::text_payload::run_text_payload;
use crate::pr::update_body::run_update_body;

pub(crate) fn run(args: &[String]) -> i32 {
    match parse(args) {
        Ok(PrAction::Help) => {
            print_usage();
            0
        }
        Ok(PrAction::BreakingDetect(opts)) => run_breaking_detect(opts),
        Ok(PrAction::BodyContext(opts)) => run_body_context(opts),
        Ok(PrAction::ChildPrRefs(opts)) => run_child_pr_refs(opts),
        Ok(PrAction::Directives(opts)) => run_directives(opts),
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
        Ok(PrAction::UpdateBody(opts)) => run_update_body(opts),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}

fn run_directives(opts: crate::pr::commands::pr_directives_options::PrDirectivesOptions) -> i32 {
    let records = scan_directives(&opts.text, opts.unique);
    match opts.format {
        PrDirectivesFormat::Plain => {
            emit_plain(&records);
            0
        }
        PrDirectivesFormat::Json => emit_json(&records),
    }
}
