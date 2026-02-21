// projects/products/unstable/autonomous_dev_ai/src/persistence/memory_state_index.rs
use serde::{Deserialize, Serialize};

use crate::memory_graph::MemoryGraph;

use super::utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStateIndex {
    pub generated_at_secs: u64,
    pub explored_files_count: usize,
    pub plans_count: usize,
    pub decisions_count: usize,
    pub failures_count: usize,
    pub objective_evaluations_count: usize,
    pub metadata_keys_count: usize,
    pub max_iteration_seen: usize,
}

impl MemoryStateIndex {
    pub fn from_memory(memory: &MemoryGraph) -> Self {
        let max_iteration_seen = memory
            .plans
            .iter()
            .map(|p| p.iteration)
            .chain(memory.decisions.iter().map(|d| d.iteration))
            .chain(memory.failures.iter().map(|f| f.iteration))
            .chain(memory.objective_evaluations.iter().map(|o| o.iteration))
            .max()
            .unwrap_or(0);

        Self {
            generated_at_secs: utils::now_secs(),
            explored_files_count: memory.explored_files.len(),
            plans_count: memory.plans.len(),
            decisions_count: memory.decisions.len(),
            failures_count: memory.failures.len(),
            objective_evaluations_count: memory.objective_evaluations.len(),
            metadata_keys_count: memory.metadata.len(),
            max_iteration_seen,
        }
    }
}
