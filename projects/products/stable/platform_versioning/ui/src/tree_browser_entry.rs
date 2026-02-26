// projects/products/stable/platform_versioning/ui/src/tree_browser_entry.rs
use serde::{Deserialize, Serialize};

/// A single entry shown in the tree browser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeBrowserEntry {
    /// The entry name (file or directory name).
    pub name: String,
    /// Whether this entry is a directory.
    pub is_dir: bool,
    /// The content address of the blob or tree (as hex string).
    pub object_id: String,
}
