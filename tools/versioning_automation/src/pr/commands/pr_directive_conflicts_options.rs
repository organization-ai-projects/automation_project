//! tools/versioning_automation/src/pr/commands/pr_directive_conflicts_options.rs
use crate::pr::domain::conflicts::ConflictReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrDirectiveConflictsOptions {
    pub(crate) text: String,
    pub(crate) source_branch_count: u32,
}

impl PrDirectiveConflictsOptions {
    pub(crate) fn run_directive_conflicts(self) -> i32 {
        let report = ConflictReport::build_conflict_report(&self.text, self.source_branch_count);
        ConflictReport::emit_plain(&report);
        0
    }
}
