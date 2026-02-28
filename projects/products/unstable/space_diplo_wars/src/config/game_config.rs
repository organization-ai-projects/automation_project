use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub turns: u64,
    pub ticks_per_turn: u64,
    pub seed: u64,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            turns: 10,
            ticks_per_turn: 4,
            seed: 42,
        }
    }
}
