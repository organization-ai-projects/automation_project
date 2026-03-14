use crate::dataset_engine::DatasetTrainingBuildOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoImprovementPolicy {
    pub min_dataset_entries: usize,
    pub min_success_ratio: f64,
    pub min_average_score: Option<f64>,
    pub training_build_options: DatasetTrainingBuildOptions,
    #[serde(default = "default_trainer_trigger_min_retry_delay_seconds")]
    pub trainer_trigger_min_retry_delay_seconds: u64,
    #[serde(default = "default_trainer_trigger_max_delivery_attempts_before_dead_letter")]
    pub trainer_trigger_max_delivery_attempts_before_dead_letter: u32,
}

impl AutoImprovementPolicy {
    pub fn with_min_dataset_entries(mut self, min_dataset_entries: usize) -> Self {
        self.min_dataset_entries = min_dataset_entries;
        self
    }

    pub fn with_min_success_ratio(mut self, min_success_ratio: f64) -> Self {
        self.min_success_ratio = min_success_ratio;
        self
    }

    pub fn with_min_average_score(mut self, min_average_score: Option<f64>) -> Self {
        self.min_average_score = min_average_score;
        self
    }

    pub fn with_training_build_options(
        mut self,
        training_build_options: DatasetTrainingBuildOptions,
    ) -> Self {
        self.training_build_options = training_build_options;
        self
    }

    pub fn with_trainer_trigger_min_retry_delay_seconds(
        mut self,
        trainer_trigger_min_retry_delay_seconds: u64,
    ) -> Self {
        self.trainer_trigger_min_retry_delay_seconds = trainer_trigger_min_retry_delay_seconds;
        self
    }

    pub fn with_trainer_trigger_max_delivery_attempts_before_dead_letter(
        mut self,
        trainer_trigger_max_delivery_attempts_before_dead_letter: u32,
    ) -> Self {
        self.trainer_trigger_max_delivery_attempts_before_dead_letter =
            trainer_trigger_max_delivery_attempts_before_dead_letter.max(1);
        self
    }
}

impl Default for AutoImprovementPolicy {
    fn default() -> Self {
        Self {
            min_dataset_entries: 64,
            min_success_ratio: 0.70,
            min_average_score: Some(0.60),
            training_build_options: DatasetTrainingBuildOptions::default(),
            trainer_trigger_min_retry_delay_seconds:
                default_trainer_trigger_min_retry_delay_seconds(),
            trainer_trigger_max_delivery_attempts_before_dead_letter:
                default_trainer_trigger_max_delivery_attempts_before_dead_letter(),
        }
    }
}

fn default_trainer_trigger_min_retry_delay_seconds() -> u64 {
    30
}

fn default_trainer_trigger_max_delivery_attempts_before_dead_letter() -> u32 {
    8
}
