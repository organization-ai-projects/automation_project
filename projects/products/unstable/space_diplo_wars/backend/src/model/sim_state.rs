use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::diplomacy::treaty::Treaty;
use crate::diplomacy::treaty_offer::TreatyOffer;
use crate::fleets::fleet::Fleet;
use crate::map::star_map::StarMap;
use crate::model::empire::Empire;
use crate::model::empire_id::EmpireId;
use crate::model::fleet_id::FleetId;
use crate::model::game_id::GameId;
use crate::queues::build_queue::BuildQueue;
use crate::queues::research_queue::ResearchQueue;
use crate::time::phase::Phase;
use crate::time::tick::Tick;
use crate::time::turn::Turn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimState {
    pub game_id: GameId,
    pub current_tick: Tick,
    pub current_turn: Turn,
    pub current_phase: Phase,
    pub empires: BTreeMap<EmpireId, Empire>,
    pub build_queues: BTreeMap<EmpireId, BuildQueue>,
    pub research_queues: BTreeMap<EmpireId, ResearchQueue>,
    pub star_map: StarMap,
    pub fleets: BTreeMap<FleetId, Fleet>,
    pub treaties: BTreeMap<String, Treaty>,
    pub pending_treaty_offers: BTreeMap<String, TreatyOffer>,
}

impl SimState {
    pub fn new(game_id: GameId, star_map: StarMap) -> Self {
        Self {
            game_id,
            current_tick: Tick(0),
            current_turn: Turn(0),
            current_phase: Phase::EconomyTick,
            empires: BTreeMap::new(),
            build_queues: BTreeMap::new(),
            research_queues: BTreeMap::new(),
            star_map,
            fleets: BTreeMap::new(),
            treaties: BTreeMap::new(),
            pending_treaty_offers: BTreeMap::new(),
        }
    }
}
