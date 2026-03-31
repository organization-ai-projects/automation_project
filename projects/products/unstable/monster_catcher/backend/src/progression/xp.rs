use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct XpGain {
    pub base_yield: u32,
    pub enemy_level: u32,
    pub xp_gained: u64,
}

impl XpGain {
    pub fn compute(base_yield: u32, enemy_level: u32) -> Self {
        let xp_gained = (base_yield as u64 * enemy_level as u64) / 7;
        Self {
            base_yield,
            enemy_level,
            xp_gained,
        }
    }
}
