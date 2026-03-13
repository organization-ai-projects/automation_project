use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PathClassification {
    Stable,
    Unstable,
    Tool,
    Other,
}

impl PathClassification {
    pub fn from_product_path(path: &Path) -> Self {
        let txt = path.to_string_lossy();
        if txt.contains("projects/products/stable/") {
            Self::Stable
        } else if txt.contains("projects/products/unstable/") {
            Self::Unstable
        } else {
            Self::Other
        }
    }

    pub fn from_tool_path(path: &Path) -> Self {
        let txt = path.to_string_lossy();
        if txt.contains("/tools/") || txt.starts_with("tools/") || txt == "tools" {
            Self::Tool
        } else {
            Self::Other
        }
    }
}
