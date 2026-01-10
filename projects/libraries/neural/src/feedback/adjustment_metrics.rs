#[derive(Debug, Clone)]
pub struct AdjustmentMetrics {
    pub total_examples: usize,
    pub batch_losses: Vec<f64>,
    pub avg_loss: f64,
}

impl AdjustmentMetrics {
    pub fn new() -> Self {
        Self {
            total_examples: 0,
            batch_losses: Vec::new(),
            avg_loss: 0.0,
        }
    }
}

impl Default for AdjustmentMetrics {
    fn default() -> Self {
        Self::new()
    }
}
