use crate::domain::report_model::ReportModel;
use crate::domain::snapshot_model::SnapshotModel;
use crate::replay::replay_error::ReplayError;
use crate::replay::replay_result::ReplayResult;
use crate::replay::replay_runner::ReplayRunner;

pub struct ReplaySimulation;

pub struct ReplayOutput {
    pub report: ReportModel,
    pub snapshot: SnapshotModel,
    pub replay_result: ReplayResult,
}

impl ReplaySimulation {
    pub fn execute(journal_path: &str) -> Result<ReplayOutput, ReplayError> {
        let (report, snapshot, replay_result) = ReplayRunner::replay_from_file(journal_path)?;
        Ok(ReplayOutput {
            report,
            snapshot,
            replay_result,
        })
    }
}
