use crate::domain::journal_event::JournalEvent;
use crate::infrastructure::artifact_reader::ArtifactReader;
use crate::infrastructure::artifact_writer::ArtifactWriter;
use crate::reporting::deterministic_serializer::DeterministicSerializer;

pub struct JournalPersistence;

impl JournalPersistence {
    pub fn save(path: &str, events: &[JournalEvent]) -> Result<(), String> {
        let content = DeterministicSerializer::serialize_canonical(&events)?;
        ArtifactWriter::write(path, &content)
    }

    pub fn load(path: &str) -> Result<Vec<JournalEvent>, String> {
        let content = ArtifactReader::read(path)?;
        common_json::from_str(&content).map_err(|e| format!("Failed to parse journal: {e}"))
    }
}
