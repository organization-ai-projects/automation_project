/// Unique document identifier (deterministic: derived from path).
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub(crate) struct DocId(pub(crate) String);

impl DocId {
    pub(crate) fn from_path(path: &str) -> Self {
        Self(path.to_string())
    }
}
