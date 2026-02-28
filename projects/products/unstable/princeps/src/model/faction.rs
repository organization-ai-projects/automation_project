use serde::{Deserialize, Serialize};
use crate::model::candidate_id::CandidateId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub id: String,
    pub name: String,
    pub candidates: Vec<CandidateId>,
}

impl Faction {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            candidates: Vec::new(),
        }
    }
}
