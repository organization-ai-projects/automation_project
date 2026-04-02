use crate::combat::AbilityId;
use crate::loot::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Player {
    pub(crate) hp: u32,
    pub(crate) max_hp: u32,
    pub(crate) attack: u32,
    pub(crate) defense: u32,
    pub(crate) abilities: Vec<AbilityId>,
    pub(crate) equipped_items: Vec<ItemId>,
}

impl Player {
    pub(crate) fn is_alive(&self) -> bool {
        self.hp > 0
    }
}
