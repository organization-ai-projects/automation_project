use serde::Deserialize;

use super::create_commit_file::CreateCommitFile;

#[derive(Debug, Deserialize)]
pub struct CreateCommitRequest {
    pub author: String,
    pub message: String,
    pub timestamp_secs: Option<u64>,
    pub extra_parent: Option<String>,
    pub files: Vec<CreateCommitFile>,
}
