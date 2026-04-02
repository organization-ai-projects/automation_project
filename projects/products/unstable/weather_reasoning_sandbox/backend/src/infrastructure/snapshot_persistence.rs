use crate::domain::snapshot_model::SnapshotModel;
use crate::infrastructure::artifact_writer::ArtifactWriter;
use crate::reporting::deterministic_serializer::DeterministicSerializer;

pub struct SnapshotPersistence;

impl SnapshotPersistence {
    pub fn save(path: &str, snapshot: &SnapshotModel) -> Result<(), String> {
        let content = DeterministicSerializer::serialize_canonical(snapshot)?;
        ArtifactWriter::write(path, &content)
    }
}
