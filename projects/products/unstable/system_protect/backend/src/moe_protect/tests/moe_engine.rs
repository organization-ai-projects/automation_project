use crate::moe_protect::moe_engine::MoeEngine;
use crate::moe_protect::threat_event::ThreatEvent;
use crate::moe_protect::threat_id::ThreatId;
use crate::moe_protect::threat_level::ThreatLevel;
use crate::moe_protect::threat_type::ThreatType;

fn make_engine() -> MoeEngine {
    let mut engine = MoeEngine::new();
    engine.register_default_experts();
    engine
}

#[test]
fn engine_registers_three_default_experts() {
    let engine = make_engine();
    let experts = engine.list_experts();
    assert_eq!(experts.len(), 3);
}

#[test]
fn engine_analyzes_virus_threat() {
    let mut engine = make_engine();
    let event = ThreatEvent::new(
        ThreatType::Virus,
        ThreatLevel::High,
        "src",
        "dst",
        "payload",
    )
    .with_id(ThreatId::from_str("test-engine-001"));
    let result = engine.analyze(event);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(!result.verdicts.is_empty());
}

#[test]
fn engine_analyzes_network_intrusion() {
    let mut engine = make_engine();
    let event = ThreatEvent::new(
        ThreatType::NetworkIntrusion,
        ThreatLevel::Critical,
        "src",
        "dst",
        "data",
    )
    .with_id(ThreatId::from_str("test-engine-002"));
    let result = engine.analyze(event);
    assert!(result.is_ok());
}

#[test]
fn engine_status_reflects_analysis() {
    let mut engine = make_engine();
    let event = ThreatEvent::new(
        ThreatType::Virus,
        ThreatLevel::High,
        "src",
        "dst",
        "encrypt_all_files",
    )
    .with_id(ThreatId::from_str("test-engine-003"));
    let _ = engine.analyze(event);
    let status = engine.status();
    assert_eq!(status.events_analyzed, 1);
    assert_eq!(status.expert_count, 3);
}
