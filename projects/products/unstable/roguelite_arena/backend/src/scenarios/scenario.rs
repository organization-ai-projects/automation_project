use crate::scenarios::WaveTemplate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Scenario {
    pub(crate) name: String,
    pub(crate) player_hp: u32,
    pub(crate) player_attack: u32,
    pub(crate) player_defense: u32,
    pub(crate) waves: Vec<WaveTemplate>,
}
