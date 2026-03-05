use crate::snapshot::state_snapshot::StateSnapshot;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct SnapshotHash(pub String);

impl SnapshotHash {
    pub fn compute(snapshot: &StateSnapshot) -> Self {
        let state = &snapshot.state;

        let colonists = state
            .colonists
            .values()
            .map(|c| {
                let assigned = c
                    .assigned_job
                    .map(|j| j.0.to_string())
                    .unwrap_or_else(|| "none".to_string());
                format!(
                    "{}:{}:{:.6}:{:.6}:{}",
                    c.id.0, c.name, c.mood.value, c.productivity, assigned
                )
            })
            .collect::<Vec<_>>()
            .join("|");

        let jobs = state
            .job_queue
            .jobs
            .values()
            .map(|j| {
                let assigned = j
                    .assigned_to
                    .map(|c| c.0.to_string())
                    .unwrap_or_else(|| "none".to_string());
                format!(
                    "{}:{:?}:{}:{}:{}",
                    j.id.0, j.kind, j.priority, j.ticks_remaining, assigned
                )
            })
            .collect::<Vec<_>>()
            .join("|");

        let inventory = state
            .inventory
            .items
            .values()
            .map(|item| format!("{}:{}:{}", item.id.0, item.kind, item.quantity))
            .collect::<Vec<_>>()
            .join("|");

        let canonical = format!(
            "tick={}#clock={}#map={}x{}#colonists={}#jobs={}#inventory={}",
            snapshot.tick.value(),
            state.clock.current().value(),
            state.map.width,
            state.map.height,
            colonists,
            jobs,
            inventory
        );

        let hash = Sha256::digest(canonical.as_bytes());
        Self(hex::encode(hash))
    }
}
