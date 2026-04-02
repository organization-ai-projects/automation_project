use crate::config::physics_config::PhysicsConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimConfig {
    pub max_ticks: u64,
    pub seed: u64,
    pub ticks_per_era: u64,
    pub physics: PhysicsConfig,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            max_ticks: 1000,
            seed: 42,
            ticks_per_era: 50,
            physics: PhysicsConfig::default(),
        }
    }
}
