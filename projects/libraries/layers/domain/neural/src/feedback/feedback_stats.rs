// projects/libraries/layers/domain/neural/src/feedback/feedback_stats.rs
#[derive(Debug, Clone)]
pub struct FeedbackStats {
    pub total: usize,
    pub correct: usize,
    pub incorrect: usize,
    pub partial: usize,
    pub accuracy: f64,
}
