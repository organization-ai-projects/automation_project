use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MergeRequest {
    pub ours: String,
    pub theirs: String,
    pub author: String,
    pub message: String,
    pub timestamp_secs: Option<u64>,
}
