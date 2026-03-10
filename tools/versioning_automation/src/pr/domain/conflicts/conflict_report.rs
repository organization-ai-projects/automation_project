use crate::pr::domain::conflicts::resolved_conflict::ResolvedConflict;
use crate::pr::domain::conflicts::unresolved_conflict::UnresolvedConflict;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConflictReport {
    pub(crate) resolved: Vec<ResolvedConflict>,
    pub(crate) unresolved: Vec<UnresolvedConflict>,
}
