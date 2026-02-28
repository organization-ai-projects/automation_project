#![allow(dead_code)]
use crate::snapshot::snapshot_id::SnapshotId;
use crate::snapshot::state_snapshot::StateSnapshot;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct SnapshotStore {
    snapshots: BTreeMap<SnapshotId, StateSnapshot>,
}

impl SnapshotStore {
    pub fn new() -> Self { Self::default() }

    pub fn store(&mut self, id: SnapshotId, snapshot: StateSnapshot) {
        self.snapshots.insert(id, snapshot);
    }

    pub fn get(&self, id: &SnapshotId) -> Option<&StateSnapshot> {
        self.snapshots.get(id)
    }
}
