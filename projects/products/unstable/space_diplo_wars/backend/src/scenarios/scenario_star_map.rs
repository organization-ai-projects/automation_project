use serde::{Deserialize, Serialize};

use crate::scenarios::scenario_route::ScenarioRoute;
use crate::scenarios::scenario_system::ScenarioSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioStarMap {
    pub systems: Vec<ScenarioSystem>,
    pub routes: Vec<ScenarioRoute>,
}
