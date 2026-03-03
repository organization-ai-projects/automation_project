use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateRepoMetadataRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
}
