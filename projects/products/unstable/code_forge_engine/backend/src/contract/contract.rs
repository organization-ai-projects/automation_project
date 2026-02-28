// projects/products/unstable/code_forge_engine/backend/src/contract/contract.rs
use crate::contract::module_spec::ModuleSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub name: String,
    pub version: String,
    pub modules: Vec<ModuleSpec>,
}

impl Contract {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            modules: vec![],
        }
    }
}
