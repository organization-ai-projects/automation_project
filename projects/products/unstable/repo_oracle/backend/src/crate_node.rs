use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrateNode {
    pub name: String,
    pub path: String,
    pub dependencies: Vec<String>,
}
