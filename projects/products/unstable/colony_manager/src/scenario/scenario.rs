use crate::model::colonist_id::ColonistId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub map_width: u32,
    pub map_height: u32,
    pub colonists: Vec<(ColonistId, String)>,
    pub event_probability: f32,
}
