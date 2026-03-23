use std::collections::HashMap;

use protocol::ProtocolId;
use serde::{Deserialize, Serialize};

use crate::moe_core::{ExpertId, TaskId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetTrainingSample {
    pub entry_id: ProtocolId,
    pub task_id: TaskId,
    pub expert_id: ExpertId,
    pub input: String,
    pub target_output: String,
    pub source_output: String,
    pub used_correction: bool,
    pub correction_reason: Option<String>,
    pub score: Option<f64>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}
