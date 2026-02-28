use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::model::candidate_id::CandidateId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Debate {
    pub day: u32,
    pub participants: Vec<CandidateId>,
    pub transcript: Vec<String>,
    pub outcomes: BTreeMap<CandidateId, f64>,
}
