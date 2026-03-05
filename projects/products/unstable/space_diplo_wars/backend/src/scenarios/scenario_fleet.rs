use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioFleet {
    pub id: String,
    pub empire_id: String,
    pub location: String,
    pub ships: BTreeMap<String, u32>,
}
