use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PathClassification {
    Stable,
    Unstable,
    Other,
}

impl PathClassification {
    pub fn from_product_path(path: &Path) -> Self {
        let txt = path.to_string_lossy();
        if txt.contains("/projects/products/stable/") {
            Self::Stable
        } else if txt.contains("/projects/products/unstable/") {
            Self::Unstable
        } else {
            Self::Other
        }
    }
}
