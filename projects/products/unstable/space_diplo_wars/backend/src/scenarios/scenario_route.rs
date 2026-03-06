use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioRoute {
    pub from: String,
    pub to: String,
    pub distance: u32,
}
