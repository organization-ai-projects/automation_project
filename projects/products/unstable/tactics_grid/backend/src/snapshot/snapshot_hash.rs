use crate::unit::unit::Unit;
use sha2::{Digest, Sha256};

pub struct SnapshotHash;

impl SnapshotHash {
    pub fn compute_units(units: &[Unit]) -> String {
        let mut hasher = Sha256::new();
        let mut sorted: Vec<&Unit> = units.iter().collect();
        sorted.sort_by_key(|u| u.id);

        for unit in sorted {
            hasher.update(
                format!(
                    "{}:{}:{}:{},{}:{}:{}",
                    unit.id.0,
                    unit.name,
                    unit.alive,
                    unit.position.x,
                    unit.position.y,
                    unit.hp,
                    unit.max_hp,
                )
                .as_bytes(),
            );
        }
        hex::encode(hasher.finalize())
    }
}
