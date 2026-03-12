use serde::{Deserialize, Serialize};

use super::DatasetTrainingSample;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetTrainingBundle {
    pub schema_version: u32,
    pub generated_at: u64,
    pub validation_ratio: f64,
    pub split_seed: u64,
    pub total_entries: usize,
    pub included_entries: usize,
    pub filtered_low_score: usize,
    pub filtered_outcome: usize,
    pub filtered_missing_failure_correction: usize,
    pub train_samples: Vec<DatasetTrainingSample>,
    pub validation_samples: Vec<DatasetTrainingSample>,
}
