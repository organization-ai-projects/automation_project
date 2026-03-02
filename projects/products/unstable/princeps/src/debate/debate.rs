use crate::model::candidate_id::CandidateId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Debate {
    pub day: u32,
    pub participants: Vec<CandidateId>,
    pub transcript: Vec<String>,
    pub outcomes: BTreeMap<CandidateId, f64>,
}
