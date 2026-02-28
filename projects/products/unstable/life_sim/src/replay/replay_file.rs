use crate::config::SimConfig;
use crate::model::World;
use crate::sim::SimEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub seed: u64,
    pub config: SimConfig,
    pub initial_world: World,
    pub events: Vec<SimEvent>,
}
