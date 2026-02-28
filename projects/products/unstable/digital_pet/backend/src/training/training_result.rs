// projects/products/unstable/digital_pet/backend/src/training/training_result.rs
use crate::training::training_kind::TrainingKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingResult {
    pub kind: TrainingKind,
    pub stat_gain: u32,
}
