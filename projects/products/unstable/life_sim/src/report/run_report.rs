use crate::report::agent_report::AgentReport;
use crate::report::run_hash::RunHash;
use crate::report::world_snapshot::WorldSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReport {
    pub seed: u64,
    pub ticks_simulated: u64,
    pub agents: Vec<AgentReport>,
    pub snapshot: WorldSnapshot,
    pub run_hash: RunHash,
}
