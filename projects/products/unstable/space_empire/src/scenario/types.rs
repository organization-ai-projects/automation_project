use crate::build::BuildingKind;
use crate::economy::ResourceKind;
use crate::model::{EmpireId, PlanetId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub empires: Vec<EmpireSetup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpireSetup {
    pub empire_id: EmpireId,
    pub planets: Vec<PlanetSetup>,
    pub starting_resources: BTreeMap<ResourceKind, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetSetup {
    pub planet_id: PlanetId,
    pub building_levels: BTreeMap<BuildingKind, u32>,
}
