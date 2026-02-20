// projects/libraries/layers/domain/neural/src/training/training_metrics.rs
#[derive(Debug)]
pub struct TrainingMetrics {
    pub train_losses: Vec<f64>,
    pub val_losses: Vec<f64>,
    pub best_epoch: usize,
    pub final_loss: f64,
}
