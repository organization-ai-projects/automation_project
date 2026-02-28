use crate::build::{BuildQueue, BuildingKind};
use crate::economy::ResourceWallet;
use crate::model::{EmpireId, FleetId, PlanetId};
use crate::research::{ResearchQueue, TechKind};
use crate::ships::Fleet;
use crate::time::Tick;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimState {
    pub tick: Tick,
    pub empires: BTreeMap<EmpireId, EmpireState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpireState {
    pub empire_id: EmpireId,
    pub planets: BTreeMap<PlanetId, PlanetState>,
    pub wallet: ResourceWallet,
    pub build_queue: BuildQueue,
    pub research_queue: ResearchQueue,
    pub fleets: BTreeMap<FleetId, Fleet>,
    pub researched_techs: BTreeSet<TechKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetState {
    pub planet_id: PlanetId,
    pub empire_id: EmpireId,
    pub building_levels: BTreeMap<BuildingKind, u32>,
    pub production_modifier: f64,
}
