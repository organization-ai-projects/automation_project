use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::model::candidate_id::CandidateId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterBlock {
    pub id: String,
    pub name: String,
    pub size: u32,
    pub preferences: BTreeMap<String, i32>,
    pub sensitivities: BTreeMap<String, f64>,
    pub support: BTreeMap<CandidateId, f64>,
}

impl VoterBlock {
    pub fn new(id: impl Into<String>, name: impl Into<String>, size: u32) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            size,
            preferences: BTreeMap::new(),
            sensitivities: BTreeMap::new(),
            support: BTreeMap::new(),
        }
    }
}
