// projects/products/unstable/autonomous_dev_ai/src/persistence/failure_inverted_index.rs
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::memory::FailureEntry;

use super::utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureInvertedIndex {
    pub generated_at_secs: u64,
    pub by_kind: HashMap<String, usize>,
    pub by_tool: HashMap<String, usize>,
    pub by_iteration: HashMap<usize, usize>,
    pub latest_failure_iteration: Option<usize>,
}

impl FailureInvertedIndex {
    pub fn from_failures(failures: &[FailureEntry]) -> Self {
        let mut by_kind = HashMap::new();
        let mut by_tool = HashMap::new();
        let mut by_iteration = HashMap::new();
        let mut latest_failure_iteration = None;

        for failure in failures {
            let kind = utils::infer_failure_kind(failure);
            *by_kind.entry(kind).or_insert(0) += 1;

            let tool = utils::infer_failure_tool(failure);
            *by_tool.entry(tool).or_insert(0) += 1;

            *by_iteration.entry(failure.iteration).or_insert(0) += 1;
            latest_failure_iteration = Some(
                latest_failure_iteration
                    .map(|v: usize| v.max(failure.iteration))
                    .unwrap_or(failure.iteration),
            );
        }

        Self {
            generated_at_secs: utils::now_secs(),
            by_kind,
            by_tool,
            by_iteration,
            latest_failure_iteration,
        }
    }
}
