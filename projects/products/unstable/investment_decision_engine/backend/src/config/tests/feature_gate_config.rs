use crate::config::{EngineConfig, FeatureGateConfig};

#[test]
fn from_config_mirrors_settings() {
    let mut config = EngineConfig::default();
    config.enable_recommendation_output = true;
    config.enable_neural_assistance = true;
    let gate = FeatureGateConfig::from_config(&config);
    assert!(gate.is_recommendation_allowed());
    assert!(gate.is_neural_allowed());
}

#[test]
fn disabled_gates_block_features() {
    let mut config = EngineConfig::default();
    config.enable_recommendation_output = false;
    config.enable_neural_assistance = false;
    let gate = FeatureGateConfig::from_config(&config);
    assert!(!gate.is_recommendation_allowed());
    assert!(!gate.is_neural_allowed());
}

#[test]
fn feature_gate_serialization_roundtrip() {
    let gate = FeatureGateConfig::default();
    let json = common_json::to_json_string(&gate).unwrap();
    let restored: FeatureGateConfig = common_json::from_str(&json).unwrap();
    assert_eq!(gate, restored);
}
