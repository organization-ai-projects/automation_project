use crate::pr::model::pr_auto_add_closes_options::PrAutoAddClosesOptions;
use crate::pr::model::pr_closure_marker_options::PrClosureMarkerOptions;
use crate::pr::model::pr_closure_refs_options::PrClosureRefsOptions;
use crate::pr::model::pr_directive_conflicts_options::PrDirectiveConflictsOptions;
use crate::pr::model::pr_directives_options::PrDirectivesOptions;
use crate::pr::model::pr_directives_state_options::PrDirectivesStateOptions;
use crate::pr::model::pr_issue_decision_options::PrIssueDecisionOptions;
use crate::pr::model::pr_non_closing_refs_options::PrNonClosingRefsOptions;
use crate::pr::model::pr_resolve_category_options::PrResolveCategoryOptions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PrAction {
    Help,
    Directives(PrDirectivesOptions),
    ClosureRefs(PrClosureRefsOptions),
    DirectivesState(PrDirectivesStateOptions),
    DirectiveConflicts(PrDirectiveConflictsOptions),
    IssueDecision(PrIssueDecisionOptions),
    ClosureMarker(PrClosureMarkerOptions),
    NonClosingRefs(PrNonClosingRefsOptions),
    ResolveCategory(PrResolveCategoryOptions),
    AutoAddCloses(PrAutoAddClosesOptions),
}
