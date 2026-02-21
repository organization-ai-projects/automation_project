use serde::{Deserialize, Serialize};

use crate::memory::DecisionEntry;

use super::utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionInvertedIndex {
    pub generated_at_secs: u64,
    pub by_action: std::collections::HashMap<String, usize>,
    pub by_iteration: std::collections::HashMap<usize, usize>,
    pub latest_decision_iteration: Option<usize>,
}

impl DecisionInvertedIndex {
    pub fn from_decisions(decisions: &[DecisionEntry]) -> Self {
        let mut by_action = std::collections::HashMap::new();
        let mut by_iteration = std::collections::HashMap::new();
        let mut latest_decision_iteration = None;

        for decision in decisions {
            let action = utils::infer_decision_action(decision);
            *by_action.entry(action).or_insert(0) += 1;
            *by_iteration.entry(decision.iteration).or_insert(0) += 1;
            latest_decision_iteration = Some(
                latest_decision_iteration
                    .map(|v: usize| v.max(decision.iteration))
                    .unwrap_or(decision.iteration),
            );
        }

        Self {
            generated_at_secs: utils::now_secs(),
            by_action,
            by_iteration,
            latest_decision_iteration,
        }
    }
}
