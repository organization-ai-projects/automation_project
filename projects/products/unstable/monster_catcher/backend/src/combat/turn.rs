use crate::combat::action::BattleAction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Turn {
    pub turn_number: u32,
    pub player_action: BattleAction,
    pub enemy_action: BattleAction,
    pub player_damage_dealt: u32,
    pub enemy_damage_dealt: u32,
    pub player_hp_after: u32,
    pub enemy_hp_after: u32,
    pub status_applied: Vec<String>,
}
