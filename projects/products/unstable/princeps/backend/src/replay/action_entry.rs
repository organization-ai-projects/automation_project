use crate::actions::action::Action;
use crate::model::candidate_id::CandidateId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEntry {
    pub day: u32,
    pub candidate_id: CandidateId,
    pub action: Action,
}
