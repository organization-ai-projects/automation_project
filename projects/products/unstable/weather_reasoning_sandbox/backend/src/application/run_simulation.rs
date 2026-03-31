use crate::domain::report_model::ReportModel;
use crate::domain::snapshot_model::SnapshotModel;
use crate::infrastructure::dataset_loader::DatasetLoader;
use crate::infrastructure::dataset_parser::DatasetParser;
use crate::reporting::canonical_report_builder::CanonicalReportBuilder;
use crate::reporting::canonical_snapshot_builder::CanonicalSnapshotBuilder;
use crate::simulation::simulation_engine::SimulationEngine;

pub struct RunSimulation;

pub struct RunOutput {
    pub report: ReportModel,
    pub snapshot: SnapshotModel,
    pub journal: Vec<crate::domain::journal_event::JournalEvent>,
}

impl RunSimulation {
    pub fn execute(
        seed: u64,
        tick_count: u64,
        dataset_path: Option<&str>,
    ) -> Result<RunOutput, String> {
        let (dataset_id, observations) = match dataset_path {
            Some(path) => DatasetLoader::load(path)?,
            None => DatasetLoader::load_default(seed, tick_count),
        };

        let dataset_checksum = DatasetParser::compute_checksum(&observations);

        let output =
            SimulationEngine::run(seed, tick_count, dataset_id, dataset_checksum, observations);

        let snapshot = CanonicalSnapshotBuilder::build(&output)?;
        let report = CanonicalReportBuilder::build(
            &output,
            Some(snapshot.snapshot_checksum.clone()),
            None,
        )?;

        Ok(RunOutput {
            report,
            snapshot,
            journal: output.journal,
        })
    }
}
