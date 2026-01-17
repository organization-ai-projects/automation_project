// projects/libraries/neural/src/feedback/feedback_config.rs
#[derive(Debug, Clone)]
pub struct FeedbackConfig {
    /// Learning rate for adjustments
    pub learning_rate: f64,
    /// Minimum number of feedbacks before adjustment
    pub min_feedback_count: usize,
    /// Batch size for adjustments
    pub batch_size: usize,
    /// Save history to disk
    pub save_history: bool,
    pub history_path: std::path::PathBuf,
    /// Sampling ratio for Correct feedbacks
    pub correct_sampling_ratio: f32,
}

impl Default for FeedbackConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.001, // Smaller than initial training
            min_feedback_count: 10,
            batch_size: 5,
            save_history: true,
            history_path: "data/feedback_history.json".into(),
            correct_sampling_ratio: 0.05, // Default value, adjust as needed
        }
    }
}
