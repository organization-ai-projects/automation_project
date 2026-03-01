// projects/products/unstable/digital_pet/backend/src/battle/battle_state.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleState {
    pub turn: u32,
    pub pet_hp: u32,
    pub opponent_hp: u32,
    pub finished: bool,
    pub winner: Option<String>,
    pub log: Vec<String>,
}
