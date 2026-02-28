use crate::build::BuildingKind;
use crate::economy::ResourceKind;
use crate::model::PlanetState;
use std::collections::BTreeMap;

pub struct ProductionFormula;

impl ProductionFormula {
    pub fn compute(building_level: u32, modifier: f64) -> u64 {
        ((30 * building_level as u64) as f64 * modifier) as u64
    }
}

pub struct ProductionOutput {
    pub per_tick: BTreeMap<ResourceKind, u64>,
}

impl ProductionOutput {
    pub fn compute_for_planet(planet: &PlanetState) -> ProductionOutput {
        let mut per_tick = BTreeMap::new();
        let mines = [
            (BuildingKind::MetalMine, ResourceKind::Metal),
            (BuildingKind::CrystalMine, ResourceKind::Crystal),
            (BuildingKind::DeuteriumSynthesizer, ResourceKind::Deuterium),
        ];
        for (building, resource) in mines {
            let level = *planet.building_levels.get(&building).unwrap_or(&0);
            if level > 0 {
                let amount = ProductionFormula::compute(level, planet.production_modifier);
                per_tick.insert(resource, amount);
            }
        }
        ProductionOutput { per_tick }
    }
}
