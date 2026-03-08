use super::run_hash::compute_canonical_run_hash;
use super::turn_report::TurnReport;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchReport {
    pub map_name: String,
    pub seed: u64,
    pub turns: Vec<TurnReport>,
    pub run_hash: String,
}

impl MatchReport {
    /// Build a MatchReport by computing the run_hash from the turns data using canonical JSON.
    pub fn build(map_name: String, seed: u64, turns: Vec<TurnReport>) -> Self {
        let run_hash = compute_canonical_run_hash(&turns);
        Self {
            map_name,
            seed,
            turns,
            run_hash,
        }
    }
}
