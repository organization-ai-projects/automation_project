use crate::model::candidate_id::CandidateId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSummary {
    pub seed: u64,
    pub days: u32,
    pub total_events: usize,
    pub total_debates: usize,
    pub total_polls: usize,
    pub candidate_final_approvals: BTreeMap<CandidateId, f64>,
}
