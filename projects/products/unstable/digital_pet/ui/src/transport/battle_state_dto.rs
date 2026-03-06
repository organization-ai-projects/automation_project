use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleStateDto {
    pub turn: u32,
    pub pet_hp: u32,
    pub opponent_hp: u32,
    pub finished: bool,
    pub winner: Option<String>,
    pub log: Vec<String>,
}
