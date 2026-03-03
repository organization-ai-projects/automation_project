use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UploadSummary {
    pub objects_written: usize,
    pub refs_updated: usize,
}
