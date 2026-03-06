use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioEmpire {
    pub id: String,
    pub name: String,
    pub home_system: String,
    pub resources: BTreeMap<String, i64>,
    pub tech_levels: BTreeMap<String, u32>,
}
