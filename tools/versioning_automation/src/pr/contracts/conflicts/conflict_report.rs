use crate::pr::contracts::conflicts::resolved_conflict::ResolvedConflict;
use crate::pr::contracts::conflicts::unresolved_conflict::UnresolvedConflict;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConflictReport {
    pub(crate) resolved: Vec<ResolvedConflict>,
    pub(crate) unresolved: Vec<UnresolvedConflict>,
}
