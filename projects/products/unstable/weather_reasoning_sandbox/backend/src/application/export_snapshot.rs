use crate::domain::snapshot_model::SnapshotModel;
use crate::infrastructure::snapshot_persistence::SnapshotPersistence;
use crate::reporting::deterministic_serializer::DeterministicSerializer;

pub struct ExportSnapshot;

impl ExportSnapshot {
    pub fn to_json(snapshot: &SnapshotModel) -> Result<String, String> {
        DeterministicSerializer::serialize_canonical(snapshot)
    }

    pub fn to_file(snapshot: &SnapshotModel, path: &str) -> Result<(), String> {
        SnapshotPersistence::save(path, snapshot)
    }
}
