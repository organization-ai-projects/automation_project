use common_json::Json;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Migration {
    pub from_version: u32,
    pub to_version: u32,
    #[serde(default)]
    pub renames: BTreeMap<String, String>,
    #[serde(default)]
    pub defaults: BTreeMap<String, Json>,
}
