use crate::pr::model::pr_auto_add_closes_options::PrAutoAddClosesOptions;
use crate::pr::model::pr_directive_conflicts_options::PrDirectiveConflictsOptions;
use crate::pr::model::pr_directives_options::PrDirectivesOptions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PrAction {
    Help,
    Directives(PrDirectivesOptions),
    DirectiveConflicts(PrDirectiveConflictsOptions),
    AutoAddCloses(PrAutoAddClosesOptions),
}
