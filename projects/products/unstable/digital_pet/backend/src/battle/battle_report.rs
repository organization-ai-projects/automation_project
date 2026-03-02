// projects/products/unstable/digital_pet/backend/src/battle/battle_report.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleReport {
    pub winner: String,
    pub turns: u32,
    pub pet_hp_remaining: u32,
    pub opponent_hp_remaining: u32,
}
