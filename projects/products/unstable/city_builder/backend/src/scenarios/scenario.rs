use super::checkpoint::Checkpoint;
use super::initial_road::InitialRoad;
use super::initial_service::InitialService;
use super::initial_zone::InitialZone;
use super::scripted_action::ScriptedAction;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Scenario {
    pub name: String,
    pub grid_width: u32,
    pub grid_height: u32,
    pub initial_zones: Vec<InitialZone>,
    pub initial_roads: Vec<InitialRoad>,
    pub initial_services: Vec<InitialService>,
    pub scripted_actions: Vec<ScriptedAction>,
    pub checkpoints: Vec<Checkpoint>,
}
