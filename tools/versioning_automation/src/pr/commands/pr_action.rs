use crate::pr::commands::pr_auto_add_closes_options::PrAutoAddClosesOptions;
use crate::pr::commands::pr_closure_marker_options::PrClosureMarkerOptions;
use crate::pr::commands::pr_closure_refs_options::PrClosureRefsOptions;
use crate::pr::commands::pr_directive_conflict_guard_options::PrDirectiveConflictGuardOptions;
use crate::pr::commands::pr_directive_conflicts_options::PrDirectiveConflictsOptions;
use crate::pr::commands::pr_directives_options::PrDirectivesOptions;
use crate::pr::commands::pr_directives_state_options::PrDirectivesStateOptions;
use crate::pr::commands::pr_issue_decision_options::PrIssueDecisionOptions;
use crate::pr::commands::pr_non_closing_refs_options::PrNonClosingRefsOptions;
use crate::pr::commands::pr_resolve_category_options::PrResolveCategoryOptions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PrAction {
    Help,
    Directives(PrDirectivesOptions),
    ClosureRefs(PrClosureRefsOptions),
    DirectivesState(PrDirectivesStateOptions),
    DirectiveConflicts(PrDirectiveConflictsOptions),
    DirectiveConflictGuard(PrDirectiveConflictGuardOptions),
    IssueDecision(PrIssueDecisionOptions),
    ClosureMarker(PrClosureMarkerOptions),
    NonClosingRefs(PrNonClosingRefsOptions),
    ResolveCategory(PrResolveCategoryOptions),
    AutoAddCloses(PrAutoAddClosesOptions),
}
