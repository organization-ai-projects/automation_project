use crate::combat::HitResult;
use crate::loot::ItemId;
use crate::model::EnemyId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ArenaEvent {
    WaveStarted { wave_index: u32, enemy_count: u32 },
    WaveCleared { wave_index: u32 },
    AllWavesCleared,
    PlayerAttack { target: EnemyId, result: HitResult },
    EnemyAttack { enemy_id: EnemyId, result: HitResult },
    EnemyDefeated { enemy_id: EnemyId },
    PlayerDefeated,
    AbilityUsed { ability_name: String },
    LootDropped { item_name: String, item_id: ItemId },
}
