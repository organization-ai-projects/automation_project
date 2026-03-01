use crate::model::colonist_id::ColonistId;
use crate::needs::needs_state::NeedsState;
use crate::mood::mood::Mood;
use crate::jobs::job_id::JobId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colonist {
    pub id: ColonistId,
    pub name: String,
    pub needs: NeedsState,
    pub mood: Mood,
    pub assigned_job: Option<JobId>,
    pub productivity: f32,
}

impl Colonist {
    pub fn new(id: ColonistId, name: String) -> Self {
        Self {
            id, name,
            needs: NeedsState::default(),
            mood: Mood::default(),
            assigned_job: None,
            productivity: 1.0,
        }
    }
}
