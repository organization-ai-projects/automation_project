use super::*;

#[test]
fn test_model_registry_lifecycle() {
    let mut reg = ModelRegistry::new();
    let mv = ModelVersion::new("codegen", "v2.0", "models/codegen_v2.bin", 0.8);
    reg.register(mv);

    assert_eq!(reg.state("codegen"), Some(RolloutState::Pending));
    assert!(reg.promote_to_canary("codegen"));
    assert_eq!(reg.state("codegen"), Some(RolloutState::Canary));
    assert!(reg.promote_to_production("codegen"));
    assert_eq!(reg.state("codegen"), Some(RolloutState::Production));
}

#[test]
fn test_model_rollback() {
    let mut reg = ModelRegistry::new();
    let mv = ModelVersion::new("intent", "v1.0", "models/intent_v1.bin", 0.7);
    reg.register(mv);
    reg.promote_to_canary("intent");
    reg.rollback("intent", "test rollback");
    assert_eq!(reg.state("intent"), Some(RolloutState::RolledBack));
    assert!(!reg.get("intent").expect("model exists").active);
}

#[test]
fn test_confidence_gate() {
    let gate = ConfidenceGate::new(0.75);
    assert!(gate.passes(0.9));
    assert!(gate.passes(0.75));
    assert!(!gate.passes(0.74));
}

#[test]
fn test_drift_detector_triggers() {
    let mut detector = DriftDetector::new(3, 0.7);
    // Below threshold — should trigger after window fills
    assert!(!detector.observe(0.5));
    assert!(!detector.observe(0.4));
    // Third observation fills window; avg = (0.5+0.4+0.3)/3 = 0.4 < 0.7 → drift
    assert!(detector.observe(0.3));
}

#[test]
fn test_drift_detector_no_drift() {
    let mut detector = DriftDetector::new(3, 0.7);
    assert!(!detector.observe(0.9));
    assert!(!detector.observe(0.85));
    assert!(!detector.observe(0.8));
}

#[test]
fn test_governance_symbolic_override_on_drift() {
    // Build a governance instance with a small, known window size so the
    // test is not coupled to the `DriftDetector::default()` configuration.
    let mut gov = ModelGovernance {
        registry: ModelRegistry::new(),
        confidence_gate: ConfidenceGate::new(0.7),
        drift_detector: DriftDetector::new(3, 0.7),
    };
    let mv = ModelVersion::new("codegen", "v1", "path", 0.5);
    gov.registry.register(mv);
    gov.registry.promote_to_canary("codegen");

    // Fill window with low-confidence values (avg 0.3 < threshold 0.7).
    gov.accept("codegen", 0.3);
    gov.accept("codegen", 0.3);
    // Third observation fills the 3-slot window and triggers drift → rollback.
    let result = gov.accept("codegen", 0.3);
    assert!(
        !result,
        "governance should reject suggestion and prefer symbolic override"
    );
    assert_eq!(
        gov.registry.state("codegen"),
        Some(RolloutState::RolledBack)
    );
}
