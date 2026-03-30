//! tools/versioning_automation/src/pr/execute.rs
use crate::pr::commands::{
    PrAction, PrAutoAddClosesOptions, PrBodyContextOptions, PrBreakingDetectOptions,
    PrChildPrRefsOptions, PrClosureMarkerOptions, PrClosureRefsOptions, PrDetailsOptions,
    PrDirectivesApplyOptions, PrDirectivesFormat, PrDirectivesOptions, PrDuplicateActionsOptions,
    PrEffectiveCategoryOptions, PrFieldOptions, PrGenerateDescriptionOptions,
    PrGroupByCategoryOptions, PrIssueViewOptions, PrNonClosingRefsOptions,
    PrOpenReferencingIssueOptions, PrResolveCategoryOptions, PrUpsertCommentOptions,
};
use crate::pr::issue_close_policy::run_issue_close_policy;
use crate::pr::issue_context::run_issue_context;
use crate::pr::issue_decision::run_issue_decision;
use crate::pr::issue_ref_kind::run_issue_ref_kind;
use crate::pr::normalize_issue_key::run_normalize_issue_key;
use crate::pr::parse::parse;
use crate::pr::pr_state::run_pr_state;
use crate::pr::refresh_validation::run_refresh_validation;
use crate::pr::render::print_usage;
use crate::pr::resolve_category::{run_issue_category_from_labels, run_issue_category_from_title};
use crate::pr::sort_bullets::run_sort_bullets;
use crate::pr::state::run_directives_state;
use crate::pr::text_payload::run_text_payload;
use crate::pr::update_body::run_update_body;
use crate::pr::{
    DirectiveRecord, PrDirectiveConflictGuardOptions, PrDirectiveConflictsOptions, State,
};

pub(crate) fn run(args: &[String]) -> i32 {
    match parse(args) {
        Ok(PrAction::Help) => {
            print_usage();
            0
        }
        Ok(PrAction::BreakingDetect(opts)) => PrBreakingDetectOptions::run_breaking_detect(opts),
        Ok(PrAction::BodyContext(opts)) => PrBodyContextOptions::run_body_context(opts),
        Ok(PrAction::ChildPrRefs(opts)) => PrChildPrRefsOptions::run_child_pr_refs(opts),
        Ok(PrAction::Directives(opts)) => run_directives(opts),
        Ok(PrAction::DirectivesApply(opts)) => PrDirectivesApplyOptions::run_directives_apply(opts),
        Ok(PrAction::Details(opts)) => PrDetailsOptions::run_details(opts),
        Ok(PrAction::Field(opts)) => PrFieldOptions::run_field(opts),
        Ok(PrAction::ClosureRefs(opts)) => PrClosureRefsOptions::run_closure_refs(opts),
        Ok(PrAction::DirectivesState(opts)) => run_directives_state(opts),
        Ok(PrAction::DirectiveConflicts(opts)) => {
            PrDirectiveConflictsOptions::run_directive_conflicts(opts)
        }
        Ok(PrAction::DirectiveConflictGuard(opts)) => {
            PrDirectiveConflictGuardOptions::run_directive_conflict_guard(opts)
        }
        Ok(PrAction::DuplicateActions(opts)) => {
            PrDuplicateActionsOptions::run_duplicate_actions(opts)
        }
        Ok(PrAction::EffectiveCategory(opts)) => {
            PrEffectiveCategoryOptions::run_effective_category(opts)
        }
        Ok(PrAction::GenerateDescription(opts)) => {
            PrGenerateDescriptionOptions::run_generate_description(opts)
        }
        Ok(PrAction::GroupByCategory(opts)) => {
            PrGroupByCategoryOptions::run_group_by_category(opts)
        }
        Ok(PrAction::IssueCategoryFromLabels(opts)) => run_issue_category_from_labels(opts),
        Ok(PrAction::IssueCategoryFromTitle(opts)) => run_issue_category_from_title(opts),
        Ok(PrAction::IssueClosePolicy(opts)) => run_issue_close_policy(opts),
        Ok(PrAction::IssueContext(opts)) => run_issue_context(opts),
        Ok(PrAction::IssueView(opts)) => PrIssueViewOptions::run_issue_view(opts),
        Ok(PrAction::PrState(opts)) => run_pr_state(opts),
        Ok(PrAction::RefreshValidation(opts)) => run_refresh_validation(opts),
        Ok(PrAction::IssueRefKind(opts)) => run_issue_ref_kind(opts),
        Ok(PrAction::NormalizeIssueKey(opts)) => run_normalize_issue_key(opts),
        Ok(PrAction::OpenReferencingIssue(opts)) => {
            PrOpenReferencingIssueOptions::run_open_referencing_issue(opts)
        }
        Ok(PrAction::IssueDecision(opts)) => run_issue_decision(opts),
        Ok(PrAction::ClosureMarker(opts)) => PrClosureMarkerOptions::run_closure_marker(opts),
        Ok(PrAction::NonClosingRefs(opts)) => PrNonClosingRefsOptions::run_non_closing_refs(opts),
        Ok(PrAction::ResolveCategory(opts)) => PrResolveCategoryOptions::run_resolve_category(opts),
        Ok(PrAction::SortBullets(opts)) => run_sort_bullets(opts),
        Ok(PrAction::AutoAddCloses(opts)) => PrAutoAddClosesOptions::run_auto_add_closes(opts),
        Ok(PrAction::TextPayload(opts)) => run_text_payload(opts),
        Ok(PrAction::UpdateBody(opts)) => run_update_body(opts),
        Ok(PrAction::UpsertComment(opts)) => PrUpsertCommentOptions::run_upsert_comment(opts),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}

fn run_directives(opts: PrDirectivesOptions) -> i32 {
    let records = DirectiveRecord::scan_directives(&opts.text, opts.unique);
    match opts.format {
        PrDirectivesFormat::Plain => {
            State::emit_plain(&records);
            0
        }
        PrDirectivesFormat::Json => DirectiveRecord::emit_json(&records),
    }
}
