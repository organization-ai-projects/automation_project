use crate::pr::model::pr_auto_add_closes_options::PrAutoAddClosesOptions;
use crate::pr::model::pr_closure_refs_options::PrClosureRefsOptions;
use crate::pr::model::pr_directive_conflicts_options::PrDirectiveConflictsOptions;
use crate::pr::model::pr_directives_options::PrDirectivesOptions;
use crate::pr::model::pr_directives_state_options::PrDirectivesStateOptions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PrAction {
    Help,
    Directives(PrDirectivesOptions),
    ClosureRefs(PrClosureRefsOptions),
    DirectivesState(PrDirectivesStateOptions),
    DirectiveConflicts(PrDirectiveConflictsOptions),
    AutoAddCloses(PrAutoAddClosesOptions),
}
