use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::canonical_json::CanonicalJson;
use crate::crate_graph::CrateGraph;
use crate::module_node::ModuleNode;
use crate::public_item::PublicItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub root_path: String,
    pub crate_graph: CrateGraph,
    pub modules: Vec<ModuleNode>,
    pub public_items: Vec<PublicItem>,
    pub snapshot_hash: String,
}

impl Snapshot {
    pub fn new(
        root_path: String,
        crate_graph: CrateGraph,
        mut modules: Vec<ModuleNode>,
        mut public_items: Vec<PublicItem>,
    ) -> Self {
        modules.sort();
        public_items.sort();

        let mut snapshot = Self {
            root_path,
            crate_graph,
            modules,
            public_items,
            snapshot_hash: String::new(),
        };

        let canonical = CanonicalJson::to_string(&snapshot).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        snapshot.snapshot_hash = hex::encode(hasher.finalize());

        snapshot
    }
}
