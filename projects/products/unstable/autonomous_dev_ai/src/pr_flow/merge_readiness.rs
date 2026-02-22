// projects/products/unstable/autonomous_dev_ai/src/pr_flow/merge_readiness.rs
use serde::{Deserialize, Serialize};

/// Aggregated merge-readiness verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeReadiness {
    Ready,
    NotReady { reasons: Vec<String> },
}

impl MergeReadiness {
    pub fn is_ready(&self) -> bool {
        matches!(self, MergeReadiness::Ready)
    }
}
