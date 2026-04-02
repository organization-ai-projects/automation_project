// projects/products/unstable/autonomous_dev_ai/src/objectif_score.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveScore {
    pub objective_name: String,
    pub score: f64,
    pub passed: bool,
    pub hard_constraint: bool,
}
