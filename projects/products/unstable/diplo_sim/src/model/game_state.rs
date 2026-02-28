use serde::{Deserialize, Serialize};
use super::faction::Faction;
use super::unit::Unit;
use crate::map::map_graph::MapGraph;
use crate::time::turn::Turn;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub units: Vec<Unit>,
    pub factions: Vec<Faction>,
    pub current_turn: Turn,
    pub map_graph: MapGraph,
}

impl GameState {
    pub fn unit_by_id(&self, id: super::unit_id::UnitId) -> Option<&Unit> {
        self.units.iter().find(|u| u.id == id)
    }

    pub fn unit_at(&self, territory_id: crate::map::territory_id::TerritoryId) -> Option<&Unit> {
        self.units.iter().find(|u| u.territory_id == territory_id)
    }
}
