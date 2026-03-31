use crate::model::EnemyId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Enemy {
    pub(crate) id: EnemyId,
    pub(crate) hp: u32,
    pub(crate) max_hp: u32,
    pub(crate) attack: u32,
    pub(crate) defense: u32,
}

impl Enemy {
    pub(crate) fn is_alive(&self) -> bool {
        self.hp > 0
    }
}
