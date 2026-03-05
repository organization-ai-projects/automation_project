use crate::application::snapshot_state_dto::SnapshotStateDto;

#[derive(Debug, Clone, serde::Serialize)]
pub struct SnapshotOutput {
    pub tick: u64,
    pub snapshot_hash: String,
    pub state: SnapshotStateDto,
}
