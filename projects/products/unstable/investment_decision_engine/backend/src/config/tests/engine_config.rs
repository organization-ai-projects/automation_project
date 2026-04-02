use crate::config::EngineConfig;

#[test]
fn default_config_has_expected_values() {
    let config = EngineConfig::default();
    assert!(!config.enable_neural_assistance);
    assert_eq!(config.max_scenarios, 10);
    assert!((config.confidence_threshold - 0.6).abs() < f64::EPSILON);
    assert!(config.replay_deterministic);
}

#[test]
fn config_serialization_roundtrip() {
    let config = EngineConfig::default();
    let json = common_json::to_json_string(&config).unwrap();
    let restored: EngineConfig = common_json::from_str(&json).unwrap();
    assert_eq!(config, restored);
}
