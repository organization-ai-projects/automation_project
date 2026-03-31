use crate::combat::battle_id::BattleId;
use crate::combat::turn::Turn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleReport {
    pub battle_id: BattleId,
    pub turn_count: u32,
    pub player_won: bool,
    pub turns: Vec<Turn>,
    pub xp_gained: u64,
}
