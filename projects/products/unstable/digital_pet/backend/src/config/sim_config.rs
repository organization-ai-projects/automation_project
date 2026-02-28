// projects/products/unstable/digital_pet/backend/src/config/sim_config.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimConfig {
    pub hunger_decay_per_tick: u32,
    pub fatigue_decay_per_tick: u32,
    pub happiness_decay_per_tick: u32,
    pub discipline_decay_per_tick: u32,
    pub care_mistake_threshold: u32,
    pub sickness_threshold: u32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            hunger_decay_per_tick: 1,
            fatigue_decay_per_tick: 1,
            happiness_decay_per_tick: 1,
            discipline_decay_per_tick: 1,
            care_mistake_threshold: 3,
            sickness_threshold: 5,
        }
    }
}
