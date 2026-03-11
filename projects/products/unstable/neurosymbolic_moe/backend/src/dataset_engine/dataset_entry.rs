use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::moe_core::{ExpertId, TaskId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
    Success,
    Failure,
    Partial,
    Unknown,
}

impl PartialEq for Outcome {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for Outcome {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetEntry {
    pub id: String,
    pub task_id: TaskId,
    pub expert_id: ExpertId,
    pub input: String,
    pub output: String,
    pub outcome: Outcome,
    pub score: Option<f64>,
    pub tags: Vec<String>,
    pub created_at: u64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Correction {
    pub entry_id: String,
    pub corrected_output: String,
    pub reason: String,
    pub corrected_at: u64,
}
