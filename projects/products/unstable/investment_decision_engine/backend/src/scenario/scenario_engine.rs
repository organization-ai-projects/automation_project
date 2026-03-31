use crate::assets::AssetProfile;
use crate::config::EngineConfig;
use crate::scenario::scenario_result::CaseResult;
use crate::scenario::{Scenario, ScenarioResult};

pub struct ScenarioEngine;

impl ScenarioEngine {
    pub fn evaluate(
        scenario: &Scenario,
        asset: &AssetProfile,
        _config: &EngineConfig,
    ) -> ScenarioResult {
        let current_price = asset.market_cap_usd.unwrap_or(0.0);
        let case_results: Vec<CaseResult> = scenario
            .stress_cases
            .iter()
            .map(|case| {
                let projected_price = current_price * (1.0 + case.price_change_pct);
                let projected_return = case.price_change_pct;
                let weighted = projected_return * case.probability;
                CaseResult {
                    label: case.label.clone(),
                    projected_price,
                    projected_return_pct: projected_return,
                    probability: case.probability,
                    weighted_return: weighted,
                }
            })
            .collect();

        ScenarioResult::new(&scenario.name, case_results)
    }
}
