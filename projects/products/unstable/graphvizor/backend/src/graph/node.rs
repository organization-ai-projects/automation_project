use serde::{Deserialize, Serialize};

/// A single node in a graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    pub id: String,
    pub label: Option<String>,
}
