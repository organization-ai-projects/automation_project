// projects/products/unstable/code_forge_engine/backend/src/contract/module_spec.rs
use crate::contract::file_spec::FileSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleSpec {
    pub name: String,
    pub files: Vec<FileSpec>,
}
