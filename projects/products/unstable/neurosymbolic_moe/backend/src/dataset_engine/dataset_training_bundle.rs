use serde::{Deserialize, Serialize};

use super::DatasetTrainingSample;

const DATASET_TRAINING_BUNDLE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetTrainingBundle {
    #[serde(default = "DatasetTrainingBundle::schema_version")]
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

impl DatasetTrainingBundle {
    pub fn schema_version() -> u32 {
        DATASET_TRAINING_BUNDLE_SCHEMA_VERSION
    }

    pub fn has_supported_schema(&self) -> bool {
        self.schema_version == Self::schema_version()
    }
}
