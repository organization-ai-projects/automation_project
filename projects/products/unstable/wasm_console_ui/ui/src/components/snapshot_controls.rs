/// Describes snapshot controls for rendering.
pub struct SnapshotControls {
    pub snapshot_available: bool,
    pub snapshot_json: Option<String>,
}

impl SnapshotControls {
    pub fn new(snapshot_json: Option<String>) -> Self {
        Self {
            snapshot_available: snapshot_json.is_some(),
            snapshot_json,
        }
    }

    pub fn has_snapshot(&self) -> bool {
        self.snapshot_available
    }
}
