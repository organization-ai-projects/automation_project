use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameConfig {
    pub num_turns: u32,
    pub seed: u64,
    pub num_players: u32,
    pub map_path: String,
}

impl GameConfig {
    pub fn new(num_turns: u32, seed: u64, num_players: u32, map_path: String) -> Self {
        Self {
            num_turns,
            seed,
            num_players,
            map_path,
        }
    }
}
