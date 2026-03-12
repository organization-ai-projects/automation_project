use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetTrainingSample {
    pub entry_id: String,
    pub task_id: String,
    pub expert_id: String,
    pub input: String,
    pub target_output: String,
    pub source_output: String,
    pub used_correction: bool,
    pub correction_reason: Option<String>,
    pub score: Option<f64>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}
