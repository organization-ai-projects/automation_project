use crate::build::BuildingKind;
use crate::diagnostics::SpaceEmpireError;
use crate::economy::ResourceKind;
use crate::io::JsonCodec;
use crate::model::{EmpireId, PlanetId};
use crate::scenario::{EmpireSetup, PlanetSetup, Scenario};
use std::collections::BTreeMap;

pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn load_from_json(path: &str) -> Result<Scenario, SpaceEmpireError> {
        let content = std::fs::read_to_string(path)?;
        JsonCodec::decode(&content)
    }

    pub fn default_scenario() -> Scenario {
        let mut empire1_resources = BTreeMap::new();
        empire1_resources.insert(ResourceKind::Metal, 500);
        empire1_resources.insert(ResourceKind::Crystal, 500);
        empire1_resources.insert(ResourceKind::Deuterium, 0);

        let mut empire2_resources = BTreeMap::new();
        empire2_resources.insert(ResourceKind::Metal, 500);
        empire2_resources.insert(ResourceKind::Crystal, 500);
        empire2_resources.insert(ResourceKind::Deuterium, 0);

        let mut planet1_buildings = BTreeMap::new();
        planet1_buildings.insert(BuildingKind::MetalMine, 1);
        planet1_buildings.insert(BuildingKind::CrystalMine, 1);

        let mut planet2_buildings = BTreeMap::new();
        planet2_buildings.insert(BuildingKind::MetalMine, 1);
        planet2_buildings.insert(BuildingKind::CrystalMine, 1);

        Scenario {
            empires: vec![
                EmpireSetup {
                    empire_id: EmpireId(1),
                    planets: vec![PlanetSetup {
                        planet_id: PlanetId(1),
                        building_levels: planet1_buildings,
                    }],
                    starting_resources: empire1_resources,
                },
                EmpireSetup {
                    empire_id: EmpireId(2),
                    planets: vec![PlanetSetup {
                        planet_id: PlanetId(2),
                        building_levels: planet2_buildings,
                    }],
                    starting_resources: empire2_resources,
                },
            ],
        }
    }

    pub fn scenario_hash(scenario: &Scenario) -> String {
        use sha2::{Digest, Sha256};
        let json = JsonCodec::encode(scenario).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        hex::encode(hasher.finalize())
    }
}
