use crate::scenario::scenario_loader::ScenarioLoader;

#[test]
fn load_from_str_valid() {
    let json = r#"{
        "name": "basic",
        "companies": [],
        "contracts": [],
        "pricing_policy": { "markup_percent": 30, "discount_threshold": 100, "discount_percent": 10 },
        "demand_model": { "elasticity": 1.5, "base_price_reference": 1000 },
        "segments": []
    }"#;
    let result = ScenarioLoader::load_from_str(json);
    assert!(result.is_ok());
}

#[test]
fn load_from_str_invalid() {
    let result = ScenarioLoader::load_from_str("not json");
    assert!(result.is_err());
}
