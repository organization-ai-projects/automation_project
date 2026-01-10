#[derive(Debug, Clone)]
pub struct FeedbackConfig {
    /// Learning rate pour les ajustements
    pub learning_rate: f64,
    /// Nombre minimum de feedbacks avant ajustement
    pub min_feedback_count: usize,
    /// Batch size pour les ajustements
    pub batch_size: usize,
    /// Sauvegarder l'historique sur disque
    pub save_history: bool,
    pub history_path: std::path::PathBuf,
}

impl Default for FeedbackConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.001, // Plus petit que training initial
            min_feedback_count: 10,
            batch_size: 5,
            save_history: true,
            history_path: "data/feedback_history.json".into(),
        }
    }
}