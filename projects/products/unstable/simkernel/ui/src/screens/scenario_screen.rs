#![allow(dead_code)]
pub struct ScenarioScreen { pub scenario_name: String }
impl ScenarioScreen {
    pub fn render(&self) -> String { format!("Scenario: {}", self.scenario_name) }
}
