use serde::{Deserialize, Serialize};

use crate::crate_node::CrateNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateGraph {
    pub crates: Vec<CrateNode>,
}

impl CrateGraph {
    pub fn new(mut crates: Vec<CrateNode>) -> Self {
        crates.sort();
        Self { crates }
    }

    pub fn reverse_deps(&self, crate_name: &str) -> Vec<&CrateNode> {
        let mut result: Vec<&CrateNode> = self
            .crates
            .iter()
            .filter(|c| c.dependencies.iter().any(|d| d == crate_name))
            .collect();
        result.sort_by_key(|c| &c.name);
        result
    }

    pub fn direct_deps(&self, crate_name: &str) -> Option<&[String]> {
        self.crates
            .iter()
            .find(|c| c.name == crate_name)
            .map(|c| c.dependencies.as_slice())
    }
}
