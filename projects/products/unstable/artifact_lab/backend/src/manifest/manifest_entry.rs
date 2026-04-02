#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ManifestEntry {
    pub hash: String,
    pub path: String,
    pub size: u64,
}
