use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct WaveTemplate {
    pub(crate) enemy_count: u32,
    pub(crate) enemy_hp: u32,
    pub(crate) enemy_attack: u32,
    pub(crate) enemy_defense: u32,
}
