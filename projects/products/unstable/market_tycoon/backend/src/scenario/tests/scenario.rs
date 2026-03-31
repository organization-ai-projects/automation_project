use crate::scenario::scenario::Scenario;

#[test]
fn scenario_roundtrip() {
    let json = r#"{
        "name": "test",
        "companies": [],
        "contracts": [],
        "pricing_policy": { "markup_percent": 30, "discount_threshold": 100, "discount_percent": 10 },
        "demand_model": { "elasticity": 1.5, "base_price_reference": 1000 },
        "segments": []
    }"#;
    let s: Scenario = common_json::from_str(json).unwrap();
    assert_eq!(s.name, "test");
}
