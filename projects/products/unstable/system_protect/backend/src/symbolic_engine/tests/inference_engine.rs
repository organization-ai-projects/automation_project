use crate::moe_protect::protection_action::ProtectionAction;
use crate::moe_protect::threat_event::ThreatEvent;
use crate::moe_protect::threat_id::ThreatId;
use crate::moe_protect::threat_level::ThreatLevel;
use crate::moe_protect::threat_type::ThreatType;
use crate::symbolic_engine::inference_engine::InferenceEngine;

fn make_event(threat_type: ThreatType, threat_level: ThreatLevel) -> ThreatEvent {
    ThreatEvent::new(threat_type, threat_level, "src", "dst", "payload")
        .with_id(ThreatId::from_str("test-sym-001"))
}

#[test]
fn infers_quarantine_for_high_severity_virus() {
    let engine = InferenceEngine::with_defaults();
    let event = make_event(ThreatType::Virus, ThreatLevel::Critical);
    let result = engine.infer(&event);
    assert!(result.is_some());
    let (action, confidence, _) = result.unwrap();
    assert_eq!(action, ProtectionAction::Quarantine);
    assert!(confidence > 0.9);
}

#[test]
fn infers_block_for_brute_force_medium() {
    let engine = InferenceEngine::with_defaults();
    let event = make_event(ThreatType::BruteForce, ThreatLevel::Medium);
    let result = engine.infer(&event);
    assert!(result.is_some());
    let (action, _, _) = result.unwrap();
    assert_eq!(action, ProtectionAction::Block);
}

#[test]
fn infers_alert_for_port_scan() {
    let engine = InferenceEngine::with_defaults();
    let event = make_event(ThreatType::PortScan, ThreatLevel::Low);
    let result = engine.infer(&event);
    assert!(result.is_some());
    let (action, _, _) = result.unwrap();
    assert_eq!(action, ProtectionAction::Alert);
}

#[test]
fn infers_log_for_low_severity_unknown() {
    let engine = InferenceEngine::with_defaults();
    let event = make_event(ThreatType::Custom("unknown".to_string()), ThreatLevel::Info);
    let result = engine.infer(&event);
    assert!(result.is_some());
    let (action, _, _) = result.unwrap();
    assert_eq!(action, ProtectionAction::Log);
}

#[test]
fn derive_facts_produces_type_and_level_facts() {
    let event = make_event(ThreatType::Virus, ThreatLevel::High);
    let facts = InferenceEngine::derive_facts(&event);
    assert!(facts.len() >= 2);
    assert!(
        facts
            .iter()
            .any(|f| f.subject == "threat" && f.predicate == "is_type")
    );
    assert!(
        facts
            .iter()
            .any(|f| f.subject == "threat" && f.predicate == "has_level")
    );
}
