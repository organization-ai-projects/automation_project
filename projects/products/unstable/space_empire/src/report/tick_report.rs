use crate::snapshot::SnapshotHash;
use crate::time::Tick;

pub struct TickReport {
    pub tick: Tick,
    pub snapshot_hash: SnapshotHash,
    pub event_count: usize,
}
