//! tools/versioning_automation/src/pr/domain/conflicts/mod.rs
mod conflict_report;
mod resolved_conflict;
mod unresolved_conflict;

pub(crate) use conflict_report::ConflictReport;
pub(crate) use resolved_conflict::ResolvedConflict;
pub(crate) use unresolved_conflict::UnresolvedConflict;
