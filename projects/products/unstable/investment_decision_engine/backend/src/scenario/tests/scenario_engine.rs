use crate::assets::{AssetId, AssetProfile};
use crate::config::EngineConfig;
use crate::scenario::{Scenario, ScenarioEngine, StressCase};

#[test]
fn evaluate_produces_results_for_each_case() {
    let mut scenario = Scenario::new("Test", "A test scenario");
    scenario.add_stress_case(StressCase::new("Bull", 0.3, 0.2, 0.5));
    scenario.add_stress_case(StressCase::new("Bear", -0.3, -0.2, 0.5));

    let mut asset = AssetProfile::new(AssetId::new("AAPL"), "Apple Inc.");
    asset.market_cap_usd = Some(100.0);

    let config = EngineConfig::default();
    let result = ScenarioEngine::evaluate(&scenario, &asset, &config);

    assert_eq!(result.case_results.len(), 2);
    assert!((result.expected_value - 0.0).abs() < 1e-10);
}
