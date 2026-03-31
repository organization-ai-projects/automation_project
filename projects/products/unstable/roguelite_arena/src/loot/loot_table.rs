use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LootEntry {
    pub(crate) name: String,
    pub(crate) attack_bonus: u32,
    pub(crate) defense_bonus: u32,
    pub(crate) hp_bonus: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LootTable {
    pub(crate) items: Vec<LootEntry>,
}

impl LootTable {
    pub(crate) fn default_table() -> Self {
        Self {
            items: vec![
                LootEntry { name: "Iron Sword".to_string(), attack_bonus: 2, defense_bonus: 0, hp_bonus: 0 },
                LootEntry { name: "Wooden Shield".to_string(), attack_bonus: 0, defense_bonus: 2, hp_bonus: 0 },
                LootEntry { name: "Health Potion".to_string(), attack_bonus: 0, defense_bonus: 0, hp_bonus: 10 },
                LootEntry { name: "Rusty Dagger".to_string(), attack_bonus: 1, defense_bonus: 0, hp_bonus: 0 },
                LootEntry { name: "Leather Armor".to_string(), attack_bonus: 0, defense_bonus: 1, hp_bonus: 5 },
            ],
        }
    }
}
