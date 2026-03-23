//! projects/products/unstable/autonomous_dev_ai/src/persistence/action_outcome_index.rs
use std::collections;

use serde::{Deserialize, Serialize};

use crate::value_types::PassRate;
use crate::{memory_graph::MemoryGraph, models::infer_decision_action};

use super::{ActionOutcomeStats, utils};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionOutcomeIndex {
    pub generated_at: u64,
    pub by_action: collections::HashMap<String, ActionOutcomeStats>,
}

impl ActionOutcomeIndex {
    pub fn from_memory(memory: &MemoryGraph) -> Self {
        let mut outcome_by_iteration = collections::HashMap::new();
        for evaluation in &memory.objective_evaluations {
            outcome_by_iteration.insert(evaluation.iteration, evaluation.passed);
        }

        let mut by_action: collections::HashMap<String, ActionOutcomeStats> =
            collections::HashMap::new();
        for decision in &memory.decisions {
            let action = infer_decision_action(decision);
            let stats = by_action.entry(action).or_insert(ActionOutcomeStats {
                total: 0,
                passed: 0,
                failed: 0,
                unknown: 0,
                pass_rate: PassRate::new(0.0).expect("0.0 must be a valid pass rate"),
            });
            stats.total = stats.total.saturating_add(1);
            match outcome_by_iteration.get(&decision.iteration).copied() {
                Some(true) => stats.passed = stats.passed.saturating_add(1),
                Some(false) => stats.failed = stats.failed.saturating_add(1),
                None => stats.unknown = stats.unknown.saturating_add(1),
            }
        }

        for stats in by_action.values_mut() {
            let observed = stats.passed.saturating_add(stats.failed);
            let raw = if observed == 0 {
                0.0
            } else {
                stats.passed as f64 / observed as f64
            };
            stats.pass_rate = PassRate::new(raw)
                .unwrap_or_else(|| PassRate::new(0.0).expect("0.0 must be a valid pass rate"));
        }

        Self {
            generated_at: utils::now_secs(),
            by_action,
        }
    }
}
