use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AssetId(pub String);

impl fmt::Display for AssetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for AssetId {
    fn from(s: String) -> Self {
        AssetId(s)
    }
}

impl From<&str> for AssetId {
    fn from(s: &str) -> Self {
        AssetId(s.to_string())
    }
}
