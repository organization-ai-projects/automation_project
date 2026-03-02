// projects/products/unstable/hospital_tycoon/backend/src/snapshot/snapshot_hash.rs
use crate::model::hospital_state::HospitalState;
use sha2::{Digest, Sha256};

pub struct SnapshotHash;

impl SnapshotHash {
    pub fn compute(state: &HospitalState) -> String {
        let mut hasher = Sha256::new();
        hasher.update(state.tick.value().to_le_bytes());
        hasher.update((state.patients.len() as u64).to_le_bytes());
        hasher.update((state.treated_patients.len() as u64).to_le_bytes());
        hasher.update(state.budget.balance.to_le_bytes());
        hasher.update(state.reputation.score.to_le_bytes());
        hex::encode(hasher.finalize())
    }
}
