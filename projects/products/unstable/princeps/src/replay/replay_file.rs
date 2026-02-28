use crate::actions::action::Action;
use crate::model::candidate_id::CandidateId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEntry {
    pub day: u32,
    pub candidate_id: CandidateId,
    pub action: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub seed: u64,
    pub days: u32,
    pub actions: Vec<ActionEntry>,
    pub drawn_event_indices: Vec<(u32, usize)>,
}

impl ReplayFile {
    pub fn new(seed: u64, days: u32) -> Self {
        Self {
            seed,
            days,
            actions: Vec::new(),
            drawn_event_indices: Vec::new(),
        }
    }
}
