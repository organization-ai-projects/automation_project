use crate::model::colonist_id::ColonistId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColonistReport {
    pub id: ColonistId,
    pub name: String,
    pub final_mood: f32,
    pub jobs_completed: u32,
}
