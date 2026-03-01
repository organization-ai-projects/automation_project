use crate::diagnostics::error::ColonyManagerError;
use crate::model::colonist_id::ColonistId;
use crate::scenario::scenario::Scenario;
use std::path::Path;

pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn load(path: &Path) -> Result<Scenario, ColonyManagerError> {
        let data = std::fs::read_to_string(path)?;
        let scenario = serde_json::from_str(&data)?;
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
