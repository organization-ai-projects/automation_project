//! projects/products/varina/backend/src/autopilot/autopilot_report.rs
use serde::{Deserialize, Serialize};

use crate::{
    autopilot::{AutopilotMode, AutopilotPlan},
    classified_changes::ClassifiedChanges,
};

/// Structure representing an execution report of the autopilot.
/// Combines the plan, classified changes, and logs for complete tracking.
/// Execution report (plan + actions performed or refused).
#[derive(Debug, Clone, Serialize, Deserialize)] // Added Serialize to allow JSON conversion
pub struct AutopilotReport {
    pub mode: AutopilotMode,
    pub branch: String,
    pub detached_head: bool,
    pub changes: Vec<String>,
    pub classified: ClassifiedChanges,
    pub plan: AutopilotPlan,
    pub applied: bool,
    pub logs: Vec<String>,
}

impl AutopilotReport {
    /// Add a log entry to the report.
    pub fn add_log(&mut self, entry: String) {
        self.logs.push(entry);
    }
}

#[cfg(test)]
mod tests {
    use crate::autopilot::AutopilotMode;
    use crate::classified_changes::ClassifiedChanges;
    use crate::tests::test_helpers::*;

    #[test]
    fn test_autopilot_report_usage() {
        let plan = AutopilotPlanBuilder::new()
            .branch("main")
            .will_stage(vec!["file1.rs".to_string()])
            .will_commit(true)
            .commit_message("Initial commit")
            .will_push(true)
            .notes(vec!["Note 1".to_string()])
            .build();

        let report = AutopilotReportBuilder::new()
            .mode(AutopilotMode::DryRun)
            .branch("main")
            .classified(ClassifiedChanges {
                blocked: vec!["blocked_file.rs".to_string()],
                relevant: vec!["relevant_file.rs".to_string()],
                unrelated: vec!["unrelated_file.rs".to_string()],
            })
            .plan(plan)
            .applied(true)
            .logs(vec!["Log entry".to_string()])
            .build();

        assert_eq!(report.mode, AutopilotMode::DryRun);
        assert_eq!(report.branch, "main");
        assert!(!report.detached_head);
        assert!(report.changes.is_empty());
        assert!(report.applied);
        assert_eq!(report.logs.len(), 1);

        // Using the classified and plan fields
        assert_eq!(report.classified.blocked[0], "blocked_file.rs");
        assert_eq!(report.classified.relevant[0], "relevant_file.rs");
        assert_eq!(report.classified.unrelated[0], "unrelated_file.rs");

        assert_eq!(report.plan.branch, "main");
        assert!(report.plan.will_commit);
        assert_eq!(report.plan.commit_message, "Initial commit");
        assert!(report.plan.will_push);
        assert_eq!(report.plan.notes.len(), 1);
    }
}
