use crate::model::monster::Monster;
use crate::model::monster_id::MonsterId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Party {
    pub monsters: Vec<Monster>,
}

impl Party {
    pub fn add(&mut self, monster: Monster) {
        self.monsters.push(monster);
    }

    pub fn first_alive(&self) -> Option<&Monster> {
        self.monsters.iter().find(|m| !m.is_fainted())
    }

    pub fn first_alive_mut(&mut self) -> Option<&mut Monster> {
        self.monsters.iter_mut().find(|m| !m.is_fainted())
    }

    pub fn get_mut(&mut self, id: &MonsterId) -> Option<&mut Monster> {
        self.monsters.iter_mut().find(|m| m.id == *id)
    }

    pub fn all_fainted(&self) -> bool {
        self.monsters.iter().all(|m| m.is_fainted())
    }
}
