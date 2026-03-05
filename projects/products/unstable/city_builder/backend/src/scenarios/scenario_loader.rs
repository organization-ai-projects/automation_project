use super::scenario::Scenario;
use super::scripted_action::ScriptedAction;
use crate::diagnostics::city_builder_error::CityBuilderError;
use std::path::Path;

pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn load(path: &Path) -> Result<Scenario, CityBuilderError> {
        let s = std::fs::read_to_string(path)
            .map_err(|e| CityBuilderError::InvalidScenario(e.to_string()))?;
        let scenario: Scenario = common_json::from_str(&s)
            .map_err(|e| CityBuilderError::InvalidScenario(e.to_string()))?;
        Self::validate(&scenario)?;
        Ok(scenario)
    }

    pub fn validate(scenario: &Scenario) -> Result<(), CityBuilderError> {
        if scenario.grid_width == 0 || scenario.grid_height == 0 {
            return Err(CityBuilderError::InvalidScenario(
                "grid size must be > 0".to_string(),
            ));
        }

        let in_bounds = |x: u32, y: u32| x < scenario.grid_width && y < scenario.grid_height;
        for z in &scenario.initial_zones {
            if !in_bounds(z.x, z.y) {
                return Err(CityBuilderError::InvalidScenario(format!(
                    "initial zone out of bounds at ({}, {})",
                    z.x, z.y
                )));
            }
        }
        for r in &scenario.initial_roads {
            if !in_bounds(r.x1, r.y1) || !in_bounds(r.x2, r.y2) {
                return Err(CityBuilderError::InvalidScenario(format!(
                    "initial road out of bounds from ({}, {}) to ({}, {})",
                    r.x1, r.y1, r.x2, r.y2
                )));
            }
        }
        for s in &scenario.initial_services {
            if !in_bounds(s.x, s.y) {
                return Err(CityBuilderError::InvalidScenario(format!(
                    "initial service out of bounds at ({}, {})",
                    s.x, s.y
                )));
            }
        }
        for cp in &scenario.checkpoints {
            if cp.tick == 0 {
                return Err(CityBuilderError::InvalidScenario(
                    "checkpoint tick must be >= 1".to_string(),
                ));
            }
            if cp.expected_hash.is_empty() {
                return Err(CityBuilderError::InvalidScenario(
                    "checkpoint expected_hash must not be empty".to_string(),
                ));
            }
        }

        for action in &scenario.scripted_actions {
            match action {
                ScriptedAction::PlaceZone { tick, x, y, .. } => {
                    if *tick == 0 || !in_bounds(*x, *y) {
                        return Err(CityBuilderError::InvalidScenario(
                            "invalid place_zone action".to_string(),
                        ));
                    }
                }
                ScriptedAction::PlaceRoad {
                    tick,
                    x1,
                    y1,
                    x2,
                    y2,
                } => {
                    if *tick == 0 || !in_bounds(*x1, *y1) || !in_bounds(*x2, *y2) {
                        return Err(CityBuilderError::InvalidScenario(
                            "invalid place_road action".to_string(),
                        ));
                    }
                }
                ScriptedAction::PlaceService { tick, x, y, .. } => {
                    if *tick == 0 || !in_bounds(*x, *y) {
                        return Err(CityBuilderError::InvalidScenario(
                            "invalid place_service action".to_string(),
                        ));
                    }
                }
                ScriptedAction::SetTax { tick, percent } => {
                    if *tick == 0 || *percent < 0 || *percent > 500 {
                        return Err(CityBuilderError::InvalidScenario(
                            "invalid set_tax action".to_string(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
