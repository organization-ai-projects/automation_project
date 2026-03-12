use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRegression {
    pub previous_accuracy: f64,
    pub current_accuracy: f64,
    pub delta: f64,
}
