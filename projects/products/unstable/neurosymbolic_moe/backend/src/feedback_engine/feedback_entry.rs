//! projects/products/unstable/neurosymbolic_moe/backend/src/feedback_engine/feedback_entry.rs
use protocol::ProtocolId;
use serde::{Deserialize, Serialize};

use crate::{
    feedback_engine::FeedbackType,
    moe_core::{ExpertId, TaskId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackEntry {
    pub id: ProtocolId,
    pub task_id: TaskId,
    pub expert_id: ExpertId,
    pub feedback_type: FeedbackType,
    pub score: Option<f64>,
    pub comment: String,
    pub created_at: u64,
}
