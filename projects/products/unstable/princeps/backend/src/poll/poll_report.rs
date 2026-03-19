use crate::model::candidate_id::CandidateId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollReport {
    pub day: u32,
    pub results: BTreeMap<CandidateId, f64>,
    pub block_breakdown: BTreeMap<String, BTreeMap<CandidateId, f64>>,
}

impl PollReport {
    pub fn leader(&self) -> Option<&CandidateId> {
        self.results
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(id, _)| id)
    }
}
