use crate::domain::report_model::ReportModel;
use crate::infrastructure::report_persistence::ReportPersistence;
use crate::reporting::deterministic_serializer::DeterministicSerializer;

pub struct ExportReport;

impl ExportReport {
    pub fn to_json(report: &ReportModel) -> Result<String, String> {
        DeterministicSerializer::serialize_canonical(report)
    }

    pub fn to_file(report: &ReportModel, path: &str) -> Result<(), String> {
        ReportPersistence::save(path, report)
    }
}
