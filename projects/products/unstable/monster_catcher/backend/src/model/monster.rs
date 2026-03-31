use crate::data::move_id::MoveId;
use crate::data::species_id::SpeciesId;
use crate::model::monster_id::MonsterId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Monster {
    pub id: MonsterId,
    pub species_id: SpeciesId,
    pub nickname: Option<String>,
    pub level: u32,
    pub xp: u64,
    pub current_hp: u32,
    pub max_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
    pub moves: Vec<MoveId>,
    pub status: Option<String>,
}

impl Monster {
    pub fn new(
        id: MonsterId,
        species_id: SpeciesId,
        level: u32,
        base_hp: u32,
        base_attack: u32,
        base_defense: u32,
        base_speed: u32,
        moves: Vec<MoveId>,
    ) -> Self {
        let max_hp = compute_stat(base_hp, level) + level + 10;
        let attack = compute_stat(base_attack, level);
        let defense = compute_stat(base_defense, level);
        let speed = compute_stat(base_speed, level);
        Self {
            id,
            species_id,
            nickname: None,
            level,
            xp: 0,
            current_hp: max_hp,
            max_hp,
            attack,
            defense,
            speed,
            moves,
            status: None,
        }
    }

    pub fn is_fainted(&self) -> bool {
        self.current_hp == 0
    }

    pub fn apply_damage(&mut self, amount: u32) {
        self.current_hp = self.current_hp.saturating_sub(amount);
    }

    pub fn heal(&mut self, amount: u32) {
        self.current_hp = (self.current_hp + amount).min(self.max_hp);
    }
}

fn compute_stat(base: u32, level: u32) -> u32 {
    (base * 2 * level) / 100 + 5
}
