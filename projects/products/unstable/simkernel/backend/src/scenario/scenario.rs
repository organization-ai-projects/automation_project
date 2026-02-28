#![allow(dead_code)]
use crate::scenario::scenario_id::ScenarioId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub id: ScenarioId,
    pub pack_kind: String,
    pub seed: u64,
    pub ticks: u64,
    pub turns: u64,
    pub ticks_per_turn: u64,
    pub description: String,
}

impl Scenario {
    pub fn hash(&self) -> String {
        use sha2::{Digest, Sha256};
        let data = serde_json::to_string(self).unwrap_or_default();
        let mut h = Sha256::new();
        h.update(data.as_bytes());
        hex::encode(h.finalize())
    }
}
