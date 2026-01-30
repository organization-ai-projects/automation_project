// projects/libraries/neural/src/network/activation.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Activation {
    ReLU,
    Sigmoid,
    Tanh,
    Linear,
}

impl Activation {
    pub fn apply(&self, x: f64) -> f64 {
        match self {
            Activation::ReLU => x.max(0.0),
            Activation::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            Activation::Tanh => x.tanh(),
            Activation::Linear => x,
        }
    }

    pub fn derivative(&self, x: f64) -> f64 {
        match self {
            Activation::ReLU => {
                if x > 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Activation::Sigmoid => {
                let s = self.apply(x);
                s * (1.0 - s)
            }
            Activation::Tanh => 1.0 - x.tanh().powi(2),
            Activation::Linear => 1.0,
        }
    }
}
