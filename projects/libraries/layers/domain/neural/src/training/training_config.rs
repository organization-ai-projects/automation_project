// projects/libraries/layers/domain/neural/src/training/training_config.rs
#[derive(Debug, Clone)]
pub struct TrainingConfig {
    pub learning_rate: f64,
    pub epochs: usize,
    pub batch_size: usize,
    pub validation_split: f32,
    pub early_stopping_patience: Option<usize>,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.01,
            epochs: 100,
            batch_size: 32,
            validation_split: 0.2,
            early_stopping_patience: Some(10),
        }
    }
}
