use serde::{Deserialize, Serialize};

use crate::decision::DecisionSummary;
use crate::report::RunHash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionReport {
    pub ticker: String,
    pub timestamp: String,
    pub summary: DecisionSummary,
    pub run_hash: RunHash,
}

impl DecisionReport {
    pub fn new(
        ticker: impl Into<String>,
        timestamp: impl Into<String>,
        summary: DecisionSummary,
    ) -> Self {
        let ticker = ticker.into();
        let timestamp = timestamp.into();

        let hash_input = format!(
            "decision:{}:{}:{:?}:{}",
            ticker, timestamp, summary.recommended_action, summary.confidence.score,
        );
        let run_hash = RunHash::compute(&hash_input);

        Self {
            ticker,
            timestamp,
            summary,
            run_hash,
        }
    }
}
