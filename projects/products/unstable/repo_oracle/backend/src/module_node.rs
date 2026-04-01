use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleNode {
    pub crate_name: String,
    pub module_path: String,
    pub file_path: String,
    pub public_items: Vec<String>,
}
