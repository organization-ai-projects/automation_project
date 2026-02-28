use crate::report::{RunHash, TickReport};
use crate::snapshot::SnapshotHash;

#[allow(dead_code)]
pub struct SimReport {
    pub seed: u64,
    pub ticks_run: u64,
    pub scenario_hash: String,
    pub tick_reports: Vec<TickReport>,
    pub final_snapshot_hash: SnapshotHash,
    pub run_hash: RunHash,
    pub total_events: usize,
}
