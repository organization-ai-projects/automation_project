use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::model::candidate_id::CandidateId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub id: CandidateId,
    pub name: String,
    pub charisma: u8,
    pub competence: u8,
    pub integrity: u8,
    pub volatility: u8,
    pub money: i64,
    pub volunteers: i64,
    pub media: i64,
    pub positions: BTreeMap<String, i32>,
    pub approval: f64,
}

impl Candidate {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        charisma: u8,
        competence: u8,
        integrity: u8,
        volatility: u8,
    ) -> Self {
        Self {
            id: CandidateId::new(id),
            name: name.into(),
            charisma,
            competence,
            integrity,
            volatility,
            money: 1_000_000,
            volunteers: 100,
            media: 50,
            positions: BTreeMap::new(),
            approval: 0.25,
        }
    }
}
