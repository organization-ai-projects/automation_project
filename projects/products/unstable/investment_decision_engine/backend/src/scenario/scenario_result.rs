use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioResult {
    pub scenario_name: String,
    pub case_results: Vec<CaseResult>,
    pub expected_value: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaseResult {
    pub label: String,
    pub projected_price: f64,
    pub projected_return_pct: f64,
    pub probability: f64,
    pub weighted_return: f64,
}

impl ScenarioResult {
    pub fn new(scenario_name: impl Into<String>, case_results: Vec<CaseResult>) -> Self {
        let expected_value = case_results.iter().map(|c| c.weighted_return).sum();
        Self {
            scenario_name: scenario_name.into(),
            case_results,
            expected_value,
        }
    }
}
