//! projects/products/unstable/neurosymbolic_moe/backend/src/dataset_engine/dataset_entry.rs
use std::collections::HashMap;

use common_time::Timestamp;
use protocol::ProtocolId;
use serde::{Deserialize, Serialize};

use super::outcome::Outcome;
use crate::moe_core::{ExpertId, TaskId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetEntry {
    pub id: ProtocolId,
    pub task_id: TaskId,
    pub expert_id: ExpertId,
    pub input: String,
    pub output: String,
    pub outcome: Outcome,
    pub score: Option<f64>,
    pub tags: Vec<String>,
    pub created_at: Timestamp,
    pub metadata: HashMap<String, String>,
}
