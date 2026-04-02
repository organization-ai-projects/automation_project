use crate::combat::battle_id::BattleId;
use crate::combat::turn::Turn;
use crate::model::monster::Monster;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleState {
    pub id: BattleId,
    pub player_monster: Monster,
    pub enemy_monster: Monster,
    pub turns: Vec<Turn>,
    pub finished: bool,
    pub player_won: Option<bool>,
}

impl BattleState {
    pub fn new(id: BattleId, player: Monster, enemy: Monster) -> Self {
        Self {
            id,
            player_monster: player,
            enemy_monster: enemy,
            turns: Vec::new(),
            finished: false,
            player_won: None,
        }
    }
}
