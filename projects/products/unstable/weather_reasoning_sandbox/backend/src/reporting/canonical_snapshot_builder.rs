use crate::domain::checksum_value::ChecksumValue;
use crate::domain::snapshot_model::SnapshotModel;
use crate::reporting::checksum_generator::ChecksumGenerator;
use crate::reporting::deterministic_serializer::DeterministicSerializer;
use crate::simulation::simulation_engine::SimulationOutput;

pub struct CanonicalSnapshotBuilder;

impl CanonicalSnapshotBuilder {
    pub fn build(output: &SimulationOutput) -> Result<SnapshotModel, String> {
        let final_prediction = output
            .tick_reports
            .last()
            .map(|t| t.corrected_prediction.clone());

        let mut snapshot = SnapshotModel {
            metadata: output.metadata.clone(),
            final_weather_state: output.final_state.clone(),
            contradiction_count: output.contradiction_memory.len(),
            journal_event_count: output.journal.len(),
            final_corrected_prediction: final_prediction,
            snapshot_checksum: ChecksumValue::new(String::new()),
        };

        let serialized = DeterministicSerializer::serialize_canonical(&snapshot)?;
        snapshot.snapshot_checksum = ChecksumGenerator::compute(&serialized);

        Ok(snapshot)
    }
}
