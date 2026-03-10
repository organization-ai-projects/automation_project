use crate::pr::commands::pr_auto_add_closes_options::PrAutoAddClosesOptions;
use crate::pr::commands::pr_breaking_detect_options::PrBreakingDetectOptions;
use crate::pr::commands::pr_closure_marker_options::PrClosureMarkerOptions;
use crate::pr::commands::pr_closure_refs_options::PrClosureRefsOptions;
use crate::pr::commands::pr_directive_conflict_guard_options::PrDirectiveConflictGuardOptions;
use crate::pr::commands::pr_directive_conflicts_options::PrDirectiveConflictsOptions;
use crate::pr::commands::pr_directives_apply_options::PrDirectivesApplyOptions;
use crate::pr::commands::pr_directives_options::PrDirectivesOptions;
use crate::pr::commands::pr_directives_state_options::PrDirectivesStateOptions;
use crate::pr::commands::pr_duplicate_actions_options::PrDuplicateActionsOptions;
use crate::pr::commands::pr_effective_category_options::PrEffectiveCategoryOptions;
use crate::pr::commands::pr_group_by_category_options::PrGroupByCategoryOptions;
use crate::pr::commands::pr_issue_category_from_labels_options::PrIssueCategoryFromLabelsOptions;
use crate::pr::commands::pr_issue_category_from_title_options::PrIssueCategoryFromTitleOptions;
use crate::pr::commands::pr_issue_close_policy_options::PrIssueClosePolicyOptions;
use crate::pr::commands::pr_issue_context_options::PrIssueContextOptions;
use crate::pr::commands::pr_issue_decision_options::PrIssueDecisionOptions;
use crate::pr::commands::pr_issue_ref_kind_options::PrIssueRefKindOptions;
use crate::pr::commands::pr_non_closing_refs_options::PrNonClosingRefsOptions;
use crate::pr::commands::pr_normalize_issue_key_options::PrNormalizeIssueKeyOptions;
use crate::pr::commands::pr_pr_state_options::PrPrStateOptions;
use crate::pr::commands::pr_resolve_category_options::PrResolveCategoryOptions;
use crate::pr::commands::pr_sort_bullets_options::PrSortBulletsOptions;
use crate::pr::commands::pr_text_payload_options::PrTextPayloadOptions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PrAction {
    Help,
    BreakingDetect(PrBreakingDetectOptions),
    Directives(PrDirectivesOptions),
    DirectivesApply(PrDirectivesApplyOptions),
    ClosureRefs(PrClosureRefsOptions),
    DirectivesState(PrDirectivesStateOptions),
    DirectiveConflicts(PrDirectiveConflictsOptions),
    DirectiveConflictGuard(PrDirectiveConflictGuardOptions),
    DuplicateActions(PrDuplicateActionsOptions),
    EffectiveCategory(PrEffectiveCategoryOptions),
    GroupByCategory(PrGroupByCategoryOptions),
    IssueCategoryFromLabels(PrIssueCategoryFromLabelsOptions),
    IssueCategoryFromTitle(PrIssueCategoryFromTitleOptions),
    IssueClosePolicy(PrIssueClosePolicyOptions),
    IssueContext(PrIssueContextOptions),
    PrState(PrPrStateOptions),
    IssueRefKind(PrIssueRefKindOptions),
    NormalizeIssueKey(PrNormalizeIssueKeyOptions),
    IssueDecision(PrIssueDecisionOptions),
    ClosureMarker(PrClosureMarkerOptions),
    NonClosingRefs(PrNonClosingRefsOptions),
    ResolveCategory(PrResolveCategoryOptions),
    SortBullets(PrSortBulletsOptions),
    AutoAddCloses(PrAutoAddClosesOptions),
    TextPayload(PrTextPayloadOptions),
}
