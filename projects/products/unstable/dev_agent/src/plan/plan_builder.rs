use crate::diagnostics::error::AgentError;
use crate::plan::plan::Plan;
use crate::plan::task::{Task, TaskKind};
use crate::repo::file_index::FileIndex;
use runtime_core::RuntimeId;

pub struct PlanBuilder;

impl PlanBuilder {
    pub fn new() -> Self {
        Self
    }

    /// Builds a deterministic `Plan` from a `FileIndex`.
    ///
    /// Always produces the same four-task linear pipeline (Scan → Plan → Patch → Verify)
    /// for any given input, ensuring identical plan JSON for identical repo snapshots.
    pub fn build(&self, _index: &FileIndex) -> Result<Plan, AgentError> {
        let scan_id = RuntimeId::new(1);
        let plan_id = RuntimeId::new(2);
        let patch_id = RuntimeId::new(3);
        let verify_id = RuntimeId::new(4);

        let tasks = vec![
            Task::new(scan_id, "scan", TaskKind::Scan),
            Task::new(plan_id, "plan", TaskKind::Plan),
            Task::new(patch_id, "patch", TaskKind::Patch),
            Task::new(verify_id, "verify", TaskKind::Verify),
        ];
        let edges = vec![
            (scan_id, plan_id),
            (plan_id, patch_id),
            (patch_id, verify_id),
        ];
        Ok(Plan::new(tasks, edges))
    }
}

impl Default for PlanBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_produces_four_tasks() {
        let idx = FileIndex::new(vec!["a.rs".to_string()]);
        let plan = PlanBuilder::new().build(&idx).unwrap();
        assert_eq!(plan.tasks.len(), 4);
        assert_eq!(plan.edges.len(), 3);
    }

    #[test]
    fn build_is_deterministic() {
        let idx = FileIndex::new(vec!["a.rs".to_string(), "b.rs".to_string()]);
        let p1 = PlanBuilder::new().build(&idx).unwrap();
        let p2 = PlanBuilder::new().build(&idx).unwrap();
        let j1 = serde_json::to_string(&p1).unwrap();
        let j2 = serde_json::to_string(&p2).unwrap();
        assert_eq!(j1, j2);
    }

    #[test]
    fn plan_graph_is_acyclic() {
        let idx = FileIndex::new(vec![]);
        let plan = PlanBuilder::new().build(&idx).unwrap();
        assert!(!plan.to_graph().has_cycle());
    }
}
