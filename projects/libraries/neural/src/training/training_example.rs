// projects/libraries/neural/src/training/training_example.rs
use ndarray::Array1;

#[derive(Debug, Clone)]
pub struct TrainingExample {
    pub input: Array1<f64>,
    pub target: f64,
}
