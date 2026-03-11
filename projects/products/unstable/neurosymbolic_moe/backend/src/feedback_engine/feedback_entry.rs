use serde::{Deserialize, Serialize};

use crate::moe_core::{ExpertId, TaskId};

use super::feedback_type::FeedbackType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackEntry {
    pub id: String,
    pub task_id: TaskId,
    pub expert_id: ExpertId,
    pub feedback_type: FeedbackType,
    pub score: Option<f64>,
    pub comment: String,
    pub created_at: u64,
}
