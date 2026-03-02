#![allow(dead_code)]
use crate::diagnostics::error::UiError;

/// Loads test fixtures (scenario JSON strings for integration testing).
pub struct FixtureLoader;

impl FixtureLoader {
    pub fn default_scenario_json() -> &'static str {
        r#"{
  "id": "default",
  "description": "Default test scenario",
  "seed": 42,
  "visitor_count": 10,
  "entrance_node": 0,
  "nodes": [
    {"id": 0, "name": "entrance"},
    {"id": 1, "name": "coaster_area"},
    {"id": 2, "name": "shop_area"}
  ],
  "edges": [
    {"from": 0, "to": 1, "cost": 2},
    {"from": 1, "to": 2, "cost": 2},
    {"from": 0, "to": 2, "cost": 3}
  ],
  "rides": [
    {"id": 0, "kind": "Coaster", "node": 1, "capacity": 4, "ticks_per_ride": 5, "price": 10}
  ],
  "shops": [
    {"id": 0, "node": 2, "name": "Snack Bar", "price": 5}
  ],
  "initial_budget": 1000,
  "initial_reputation": 50
}"#
    }

    pub fn load_from_file(path: &str) -> Result<String, UiError> {
        std::fs::read_to_string(path).map_err(|e| UiError::Io(e.to_string()))
    }
}
