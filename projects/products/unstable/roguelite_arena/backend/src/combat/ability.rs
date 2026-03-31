use crate::combat::AbilityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Ability {
    pub(crate) id: AbilityId,
    pub(crate) name: String,
    pub(crate) damage_bonus: u32,
    pub(crate) cooldown: u32,
    pub(crate) current_cooldown: u32,
}

impl Ability {
    pub(crate) fn is_ready(&self) -> bool {
        self.current_cooldown == 0
    }

    pub(crate) fn use_ability(&mut self) {
        self.current_cooldown = self.cooldown;
    }

    pub(crate) fn tick_cooldown(&mut self) {
        if self.current_cooldown > 0 {
            self.current_cooldown -= 1;
        }
    }
}
