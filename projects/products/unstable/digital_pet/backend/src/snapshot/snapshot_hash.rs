// projects/products/unstable/digital_pet/backend/src/snapshot/snapshot_hash.rs
use crate::model::pet::Pet;
use crate::needs::needs_state::NeedsState;
use crate::time::tick::Tick;
use sha2::{Digest, Sha256};

pub struct SnapshotHash;

impl SnapshotHash {
    pub fn compute(pet: &Pet, needs: &NeedsState, tick: Tick) -> String {
        let mut hasher = Sha256::new();
        hasher.update(tick.value().to_le_bytes());
        hasher.update(pet.species.id.0.as_bytes());
        hasher.update(pet.hp.to_le_bytes());
        hasher.update(needs.hunger.to_le_bytes());
        hasher.update(needs.fatigue.to_le_bytes());
        hasher.update(needs.happiness.to_le_bytes());
        hasher.update(needs.discipline.to_le_bytes());
        hex::encode(hasher.finalize())
    }
}
