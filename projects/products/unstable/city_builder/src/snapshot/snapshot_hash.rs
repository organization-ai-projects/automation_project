use super::state_snapshot::StateSnapshot;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotHash {
    pub value: String,
}

impl SnapshotHash {
    pub fn compute(state: &StateSnapshot) -> Self {
        let mut parts: Vec<String> = Vec::new();
        for (tile, b) in &state.buildings {
            parts.push(format!("b:{}:{},{:?}:p{}", tile.x, tile.y, b.kind, b.population));
        }
        for (tile, s) in &state.service_buildings {
            parts.push(format!("s:{}:{},{:?}", tile.x, tile.y, s.kind));
        }
        parts.push(format!("budget:{}", state.budget_balance));
        let canonical = parts.join("|");

        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        let result = hasher.finalize();
        Self { value: hex::encode(result) }
    }
}
