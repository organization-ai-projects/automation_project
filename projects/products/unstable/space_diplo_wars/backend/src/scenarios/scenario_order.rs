use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioOrder {
    pub id: String,
    pub empire_id: String,
    pub kind: String,
    pub params: BTreeMap<String, String>,
}
