use crate::data::move_data::{MoveData, MoveEffect};
use crate::data::move_id::MoveId;
use crate::data::species::LearnsetEntry;
use crate::data::species::Species;
use crate::data::species_id::SpeciesId;
use crate::data::type_id::TypeId;
use crate::encounter::encounter_table::{EncounterEntry, EncounterTable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub species: Vec<Species>,
    pub moves: Vec<MoveData>,
    pub type_effectiveness: Vec<TypeEffectivenessEntry>,
    pub encounter_table: EncounterTable,
    pub starter_species_id: SpeciesId,
    pub starter_level: u32,
    pub initial_pokeballs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeEffectivenessEntry {
    pub attacker: TypeId,
    pub defender: TypeId,
    pub factor: f64,
}

impl Scenario {
    pub fn default_scenario() -> Self {
        let fire = TypeId("fire".to_string());
        let water = TypeId("water".to_string());
        let grass = TypeId("grass".to_string());
        let normal = TypeId("normal".to_string());

        let tackle = MoveData {
            id: MoveId("tackle".to_string()),
            name: "Tackle".to_string(),
            move_type: normal.clone(),
            power: 40,
            accuracy: 100,
            pp: 35,
            effect: MoveEffect::Damage,
        };
        let ember = MoveData {
            id: MoveId("ember".to_string()),
            name: "Ember".to_string(),
            move_type: fire.clone(),
            power: 40,
            accuracy: 100,
            pp: 25,
            effect: MoveEffect::Damage,
        };
        let water_gun = MoveData {
            id: MoveId("water_gun".to_string()),
            name: "Water Gun".to_string(),
            move_type: water.clone(),
            power: 40,
            accuracy: 100,
            pp: 25,
            effect: MoveEffect::Damage,
        };
        let vine_whip = MoveData {
            id: MoveId("vine_whip".to_string()),
            name: "Vine Whip".to_string(),
            move_type: grass.clone(),
            power: 45,
            accuracy: 100,
            pp: 25,
            effect: MoveEffect::Damage,
        };
        let scratch = MoveData {
            id: MoveId("scratch".to_string()),
            name: "Scratch".to_string(),
            move_type: normal.clone(),
            power: 40,
            accuracy: 100,
            pp: 35,
            effect: MoveEffect::Damage,
        };
        let flamethrower = MoveData {
            id: MoveId("flamethrower".to_string()),
            name: "Flamethrower".to_string(),
            move_type: fire.clone(),
            power: 90,
            accuracy: 100,
            pp: 15,
            effect: MoveEffect::Damage,
        };

        let flameling = Species {
            id: SpeciesId("flameling".to_string()),
            name: "Flameling".to_string(),
            primary_type: fire.clone(),
            secondary_type: None,
            base_hp: 39,
            base_attack: 52,
            base_defense: 43,
            base_speed: 65,
            capture_rate: 45,
            base_xp_yield: 64,
            learnset: vec![
                LearnsetEntry {
                    level: 1,
                    move_id: MoveId("scratch".to_string()),
                },
                LearnsetEntry {
                    level: 1,
                    move_id: MoveId("ember".to_string()),
                },
                LearnsetEntry {
                    level: 10,
                    move_id: MoveId("flamethrower".to_string()),
                },
            ],
        };

        let aquapup = Species {
            id: SpeciesId("aquapup".to_string()),
            name: "Aquapup".to_string(),
            primary_type: water.clone(),
            secondary_type: None,
            base_hp: 44,
            base_attack: 48,
            base_defense: 65,
            base_speed: 43,
            capture_rate: 45,
            base_xp_yield: 63,
            learnset: vec![
                LearnsetEntry {
                    level: 1,
                    move_id: MoveId("tackle".to_string()),
                },
                LearnsetEntry {
                    level: 1,
                    move_id: MoveId("water_gun".to_string()),
                },
            ],
        };

        let leaflet = Species {
            id: SpeciesId("leaflet".to_string()),
            name: "Leaflet".to_string(),
            primary_type: grass.clone(),
            secondary_type: None,
            base_hp: 45,
            base_attack: 49,
            base_defense: 49,
            base_speed: 45,
            capture_rate: 45,
            base_xp_yield: 64,
            learnset: vec![
                LearnsetEntry {
                    level: 1,
                    move_id: MoveId("tackle".to_string()),
                },
                LearnsetEntry {
                    level: 1,
                    move_id: MoveId("vine_whip".to_string()),
                },
            ],
        };

        let encounter_table = EncounterTable {
            entries: vec![
                EncounterEntry {
                    species_id: SpeciesId("aquapup".to_string()),
                    min_level: 3,
                    max_level: 6,
                    weight: 40,
                },
                EncounterEntry {
                    species_id: SpeciesId("leaflet".to_string()),
                    min_level: 3,
                    max_level: 6,
                    weight: 40,
                },
                EncounterEntry {
                    species_id: SpeciesId("flameling".to_string()),
                    min_level: 4,
                    max_level: 7,
                    weight: 20,
                },
            ],
        };

        let type_effectiveness = vec![
            TypeEffectivenessEntry {
                attacker: fire.clone(),
                defender: grass.clone(),
                factor: 2.0,
            },
            TypeEffectivenessEntry {
                attacker: fire.clone(),
                defender: water.clone(),
                factor: 0.5,
            },
            TypeEffectivenessEntry {
                attacker: water.clone(),
                defender: fire.clone(),
                factor: 2.0,
            },
            TypeEffectivenessEntry {
                attacker: water.clone(),
                defender: grass.clone(),
                factor: 0.5,
            },
            TypeEffectivenessEntry {
                attacker: grass.clone(),
                defender: water.clone(),
                factor: 2.0,
            },
            TypeEffectivenessEntry {
                attacker: grass.clone(),
                defender: fire.clone(),
                factor: 0.5,
            },
        ];

        Self {
            name: "default".to_string(),
            species: vec![flameling, aquapup, leaflet],
            moves: vec![tackle, ember, water_gun, vine_whip, scratch, flamethrower],
            type_effectiveness,
            encounter_table,
            starter_species_id: SpeciesId("flameling".to_string()),
            starter_level: 5,
            initial_pokeballs: 10,
        }
    }
}
