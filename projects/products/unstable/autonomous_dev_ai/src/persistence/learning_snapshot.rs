// projects/products/unstable/autonomous_dev_ai/src/persistence/learning_snapshot.rs
use serde::{Deserialize, Serialize};

use crate::memory_graph::MemoryGraph;

use super::{DecisionInvertedIndex, FailureInvertedIndex, MemoryStateIndex, utils};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSnapshot {
    pub generated_at_secs: u64,
    pub failures_count: usize,
    pub decisions_count: usize,
    pub max_iteration_seen: usize,
    pub top_failure_kind: Option<String>,
    pub top_failure_tool: Option<String>,
    pub top_decision_action: Option<String>,
}

impl LearningSnapshot {
    pub fn from_memory(memory: &MemoryGraph) -> Self {
        let failure_index = FailureInvertedIndex::from_failures(&memory.failures);
        let decision_index = DecisionInvertedIndex::from_decisions(&memory.decisions);
        let state_index = MemoryStateIndex::from_memory(memory);

        Self {
            generated_at_secs: utils::now_secs(),
            failures_count: memory.failures.len(),
            decisions_count: memory.decisions.len(),
            max_iteration_seen: state_index.max_iteration_seen,
            top_failure_kind: utils::top_entry_key(&failure_index.by_kind),
            top_failure_tool: utils::top_entry_key(&failure_index.by_tool),
            top_decision_action: utils::top_entry_key(&decision_index.by_action),
        }
    }
}
