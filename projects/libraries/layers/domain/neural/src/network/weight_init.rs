// projects/libraries/layers/domain/neural/src/network/weight_init.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WeightInit {
    Xavier,
    He,
    LeCun,
    Zero, // For debugging only
}
