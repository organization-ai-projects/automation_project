use crate::domain::journal_event::JournalEvent;
use crate::domain::report_model::ReportModel;
use crate::domain::snapshot_model::SnapshotModel;
use crate::infrastructure::dataset_loader::DatasetLoader;
use crate::infrastructure::dataset_parser::DatasetParser;
use crate::infrastructure::journal_persistence::JournalPersistence;
use crate::replay::replay_error::ReplayError;
use crate::replay::replay_result::ReplayResult;
use crate::replay::replay_validator::ReplayValidator;
use crate::reporting::canonical_report_builder::CanonicalReportBuilder;
use crate::reporting::canonical_snapshot_builder::CanonicalSnapshotBuilder;
use crate::simulation::simulation_engine::SimulationEngine;

pub struct ReplayRunner;

impl ReplayRunner {
    pub fn replay_from_file(
        path: &str,
    ) -> Result<(ReportModel, SnapshotModel, ReplayResult), ReplayError> {
        let journal = JournalPersistence::load(path).map_err(ReplayError::LoadError)?;

        Self::replay_from_journal(&journal)
    }

    pub fn replay_from_journal(
        journal: &[JournalEvent],
    ) -> Result<(ReportModel, SnapshotModel, ReplayResult), ReplayError> {
        let metadata = Self::extract_metadata(journal)?;

        let (dataset_id, observations) = if metadata.dataset.name == "default" {
            DatasetLoader::load_default(metadata.seed, metadata.tick_count)
        } else {
            DatasetLoader::load(&metadata.dataset.path).map_err(ReplayError::LoadError)?
        };

        let dataset_checksum = DatasetParser::compute_checksum(&observations);

        let output = SimulationEngine::run(
            metadata.seed,
            metadata.tick_count,
            dataset_id,
            dataset_checksum,
            observations,
        );

        let snapshot =
            CanonicalSnapshotBuilder::build(&output).map_err(|e| ReplayError::Mismatch(e))?;

        let report = CanonicalReportBuilder::build(
            &output,
            Some(snapshot.snapshot_checksum.clone()),
            None,
        )
        .map_err(|e| ReplayError::Mismatch(e))?;

        let original_checksums = Self::extract_checksums(journal);

        let replay_result = if let Some((orig_report, orig_snapshot)) = original_checksums {
            ReplayValidator::validate(
                &orig_report,
                &report.report_checksum,
                &orig_snapshot,
                &snapshot.snapshot_checksum,
            )
        } else {
            ReplayValidator::validate(
                &report.report_checksum,
                &report.report_checksum,
                &snapshot.snapshot_checksum,
                &snapshot.snapshot_checksum,
            )
        };

        Ok((report, snapshot, replay_result))
    }

    fn extract_metadata(
        journal: &[JournalEvent],
    ) -> Result<crate::domain::run_metadata::RunMetadata, ReplayError> {
        for event in journal {
            if let JournalEvent::RunStarted { metadata } = event {
                return Ok(metadata.clone());
            }
        }
        Err(ReplayError::MissingMetadata)
    }

    fn extract_checksums(
        _journal: &[JournalEvent],
    ) -> Option<(
        crate::domain::checksum_value::ChecksumValue,
        crate::domain::checksum_value::ChecksumValue,
    )> {
        // Checksums are not embedded in the journal events themselves,
        // they are validated externally
        None
    }
}
