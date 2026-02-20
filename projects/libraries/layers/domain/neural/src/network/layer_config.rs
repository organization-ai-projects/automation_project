// projects/libraries/layers/domain/neural/src/network/layer_config.rs
use serde::{Deserialize, Serialize};

use crate::network::{Activation, WeightInit};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerConfig {
    pub input_size: usize,
    pub output_size: usize,
    pub activation: Activation,
    pub weight_init: WeightInit,
}
