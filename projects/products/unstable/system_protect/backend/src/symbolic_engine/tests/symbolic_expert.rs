use crate::moe_protect::expert::ProtectionExpert;
use crate::moe_protect::expert_type::ExpertType;
use crate::moe_protect::threat_event::ThreatEvent;
use crate::moe_protect::threat_id::ThreatId;
use crate::moe_protect::threat_level::ThreatLevel;
use crate::moe_protect::threat_type::ThreatType;
use crate::symbolic_engine::symbolic_expert::SymbolicExpert;

fn make_event(threat_type: ThreatType, threat_level: ThreatLevel) -> ThreatEvent {
    ThreatEvent::new(threat_type, threat_level, "src", "dst", "payload")
        .with_id(ThreatId::from_str("test-sym-expert-001"))
}

#[test]
fn symbolic_expert_can_analyze_any_threat() {
    let expert = SymbolicExpert::new();
    let event = make_event(ThreatType::Virus, ThreatLevel::High);
    assert!(expert.can_analyze(&event));
}

#[test]
fn symbolic_expert_type_is_symbolic_analyzer() {
    let expert = SymbolicExpert::new();
    assert_eq!(expert.expert_type(), ExpertType::SymbolicAnalyzer);
}

#[test]
fn symbolic_expert_produces_verdict_for_virus() {
    let expert = SymbolicExpert::new();
    let event = make_event(ThreatType::Virus, ThreatLevel::Critical);
    let verdict = expert.analyze(&event).unwrap();
    assert!(verdict.confidence > 0.5);
}
