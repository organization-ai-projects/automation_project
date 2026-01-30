// projects/libraries/neural/src/training/mod.rs
// Module for training models
pub mod trainer;
pub mod training_config;
pub mod training_error;
pub mod training_example;
pub mod training_metrics;

pub use trainer::Trainer;
pub use training_config::TrainingConfig;
pub use training_error::TrainingError;
pub use training_example::TrainingExample;
pub use training_metrics::TrainingMetrics;

#[cfg(test)]
mod tests;
