use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioSystem {
    pub id: String,
    pub name: String,
    pub planets: Vec<String>,
    pub owner: Option<String>,
}
