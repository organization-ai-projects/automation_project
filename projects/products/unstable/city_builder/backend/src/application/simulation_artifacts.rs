use crate::replay::replay_checkpoint::ReplayCheckpoint;
use crate::report::sim_report::SimReport;
use crate::snapshot;

#[derive(Debug, Clone)]
pub struct SimulationArtifacts {
    pub report: SimReport,
    pub checkpoints: Vec<ReplayCheckpoint>,
    pub final_state: snapshot::state_snapshot::StateSnapshot,
}
