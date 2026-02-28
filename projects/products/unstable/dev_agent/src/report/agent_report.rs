use crate::patch::patch_applier::EditSummary;
use crate::plan::plan::Plan;
use crate::verify::verifier::VerifyOutcome;
use runtime_core::Event;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReport {
    pub plan: Plan,
    pub applied_edits: Vec<EditSummary>,
    pub verify_outcomes: Vec<VerifyOutcome>,
    pub event_log: Vec<Event>,
}

impl AgentReport {
    pub fn new(
        plan: Plan,
        applied_edits: Vec<EditSummary>,
        verify_outcomes: Vec<VerifyOutcome>,
        event_log: Vec<Event>,
    ) -> Self {
        Self {
            plan,
            applied_edits,
            verify_outcomes,
            event_log,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::task::TaskKind;
    use crate::verify::verify_step::VerifyStep;
    use runtime_core::RuntimeId;

    #[test]
    fn new_stores_all_fields() {
        use crate::plan::task::Task;
        let plan = Plan::new(
            vec![Task::new(RuntimeId::new(1), "scan", TaskKind::Scan)],
            vec![],
        );
        let report = AgentReport::new(plan, vec![], vec![], vec![]);
        assert_eq!(report.plan.tasks.len(), 1);
        assert!(report.applied_edits.is_empty());
        assert!(report.verify_outcomes.is_empty());
        assert!(report.event_log.is_empty());
    }

    #[test]
    fn serializes_to_valid_json() {
        use crate::plan::task::Task;
        let plan = Plan::new(
            vec![Task::new(RuntimeId::new(1), "scan", TaskKind::Scan)],
            vec![],
        );
        let step = VerifyStep::fmt();
        let outcome = VerifyOutcome {
            step,
            passed: false,
            skipped: true,
            output: None,
        };
        let report = AgentReport::new(plan, vec![], vec![outcome], vec![]);
        let json = serde_json::to_string_pretty(&report).unwrap();
        assert!(json.contains("\"plan\""));
        assert!(json.contains("\"applied_edits\""));
        assert!(json.contains("\"verify_outcomes\""));
        assert!(json.contains("\"event_log\""));
    }
}
