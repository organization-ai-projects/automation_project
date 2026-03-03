use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCommitFile {
    pub path: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub content_hex: Option<String>,
}
