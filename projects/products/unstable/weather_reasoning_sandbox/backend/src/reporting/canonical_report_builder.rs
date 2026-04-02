use crate::domain::checksum_value::ChecksumValue;
use crate::domain::report_model::ReportModel;
use crate::reporting::checksum_generator::ChecksumGenerator;
use crate::reporting::deterministic_serializer::DeterministicSerializer;
use crate::simulation::simulation_engine::SimulationOutput;

pub struct CanonicalReportBuilder;

impl CanonicalReportBuilder {
    pub fn build(
        output: &SimulationOutput,
        snapshot_checksum: Option<ChecksumValue>,
        replay_equivalence: Option<bool>,
    ) -> Result<ReportModel, String> {
        let final_prediction = output
            .tick_reports
            .last()
            .map(|t| t.corrected_prediction.clone());

        let total_violations: usize = output.tick_reports.iter().map(|t| t.violations.len()).sum();
        let total_corrections: usize = output
            .tick_reports
            .iter()
            .map(|t| t.corrections.len())
            .sum();

        let mut report = ReportModel {
            metadata: output.metadata.clone(),
            tick_reports: output.tick_reports.clone(),
            contradiction_count: output.contradiction_memory.len(),
            total_violations,
            total_corrections,
            final_corrected_prediction: final_prediction,
            report_checksum: ChecksumValue::new(String::new()),
            snapshot_checksum,
            replay_equivalence,
        };

        let serialized = DeterministicSerializer::serialize_canonical(&report)?;
        report.report_checksum = ChecksumGenerator::compute(&serialized);

        Ok(report)
    }
}
