use crate::domain::report_model::ReportModel;
use crate::infrastructure::artifact_writer::ArtifactWriter;
use crate::reporting::deterministic_serializer::DeterministicSerializer;

pub struct ReportPersistence;

impl ReportPersistence {
    pub fn save(path: &str, report: &ReportModel) -> Result<(), String> {
        let content = DeterministicSerializer::serialize_canonical(report)?;
        ArtifactWriter::write(path, &content)
    }
}
