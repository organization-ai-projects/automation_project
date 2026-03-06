use crate::diagnostics::colony_manager_error::ColonyManagerError;
use crate::io::json_codec::JsonCodec;
use crate::model::colonist_id::ColonistId;
use crate::scenarios::Scenario;
use std::path::Path;

pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn load(path: &Path) -> Result<Scenario, ColonyManagerError> {
        let scenario: Scenario = JsonCodec::load(path)?;
        if scenario.map_width == 0 || scenario.map_height == 0 {
            return Err(ColonyManagerError::InvalidScenario(
                "map dimensions must be > 0".to_string(),
            ));
        }
        if scenario.colonists.is_empty() {
            return Err(ColonyManagerError::InvalidScenario(
                "scenario must declare at least one colonist".to_string(),
            ));
        }
        Ok(scenario)
    }
    pub fn default_scenario(name: &str) -> Scenario {
        match name {
            "mood_spiral" => Scenario {
                name: "mood_spiral".to_string(),
                map_width: 5,
                map_height: 5,
                colonists: vec![
                    (ColonistId(0), "Alice".to_string()),
                    (ColonistId(1), "Bob".to_string()),
                ],
                event_probability: 0.8,
            },
            "raid_event" => Scenario {
                name: "raid_event".to_string(),
                map_width: 8,
                map_height: 8,
                colonists: vec![
                    (ColonistId(0), "Alice".to_string()),
                    (ColonistId(1), "Bob".to_string()),
                    (ColonistId(2), "Charlie".to_string()),
                ],
                event_probability: 1.0,
            },
            _ => Scenario {
                name: name.to_string(),
                map_width: 4,
                map_height: 4,
                colonists: vec![(ColonistId(0), "Alice".to_string())],
                event_probability: 0.3,
            },
        }
    }
}
