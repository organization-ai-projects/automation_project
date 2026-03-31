use crate::loot::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Item {
    pub(crate) id: ItemId,
    pub(crate) name: String,
    pub(crate) attack_bonus: u32,
    pub(crate) defense_bonus: u32,
    pub(crate) hp_bonus: u32,
}
