use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::diplomacy::treaty::Treaty;
use crate::economy::resource_wallet::ResourceWallet;
use crate::fleets::fleet::Fleet;
use crate::map::star_map::StarMap;
use crate::model::empire_id::EmpireId;
use crate::model::fleet_id::FleetId;
use crate::model::game_id::GameId;
use crate::tech::tech_tree::TechTree;
use crate::time::phase::Phase;
use crate::time::tick::Tick;
use crate::time::turn::Turn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Empire {
    pub id: EmpireId,
    pub name: String,
    pub home_system: String,
    pub resources: ResourceWallet,
    pub tech_tree: TechTree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimState {
    pub game_id: GameId,
    pub current_tick: Tick,
    pub current_turn: Turn,
    pub current_phase: Phase,
    pub empires: BTreeMap<EmpireId, Empire>,
    pub star_map: StarMap,
    pub fleets: BTreeMap<FleetId, Fleet>,
    pub treaties: BTreeMap<String, Treaty>,
}

impl SimState {
    pub fn new(game_id: GameId, star_map: StarMap) -> Self {
        Self {
            game_id,
            current_tick: Tick(0),
            current_turn: Turn(0),
            current_phase: Phase::EconomyTick,
            empires: BTreeMap::new(),
            star_map,
            fleets: BTreeMap::new(),
            treaties: BTreeMap::new(),
        }
    }
}
