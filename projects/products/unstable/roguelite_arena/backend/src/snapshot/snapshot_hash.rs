use crate::snapshot::StateSnapshot;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub(crate) struct SnapshotHash(pub(crate) String);

impl SnapshotHash {
    pub(crate) fn compute(snapshot: &StateSnapshot) -> Self {
        let state = &snapshot.state;
        let player = format!(
            "hp={}:max_hp={}:atk={}:def={}",
            state.player.hp, state.player.max_hp, state.player.attack, state.player.defense
        );

        let enemies = state
            .current_wave
            .as_ref()
            .map(|w| {
                w.enemies
                    .iter()
                    .map(|e| format!("{}:{}:{}", e.id.0, e.hp, e.attack))
                    .collect::<Vec<_>>()
                    .join("|")
            })
            .unwrap_or_default();

        let loot = state
            .loot_collected
            .iter()
            .map(|i| format!("{}:{}", i.id.0, i.name))
            .collect::<Vec<_>>()
            .join("|");

        let canonical = format!(
            "tick={}#player={}#wave_idx={}#waves_cleared={}#enemies_killed={}#enemies={}#loot={}",
            snapshot.tick.value(),
            player,
            state.wave_index,
            state.waves_cleared,
            state.enemies_killed,
            enemies,
            loot
        );

        let hash = Sha256::digest(canonical.as_bytes());
        Self(hex::encode(hash))
    }
}
