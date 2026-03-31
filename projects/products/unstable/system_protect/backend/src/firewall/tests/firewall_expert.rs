use crate::firewall::firewall_expert::FirewallExpert;
use crate::moe_protect::expert::ProtectionExpert;
use crate::moe_protect::expert_type::ExpertType;
use crate::moe_protect::protection_action::ProtectionAction;
use crate::moe_protect::threat_event::ThreatEvent;
use crate::moe_protect::threat_id::ThreatId;
use crate::moe_protect::threat_level::ThreatLevel;
use crate::moe_protect::threat_type::ThreatType;

fn make_event(threat_type: ThreatType, source: &str, target: &str) -> ThreatEvent {
    ThreatEvent::new(threat_type, ThreatLevel::High, source, target, "payload")
        .with_id(ThreatId::from_str("test-fw-001"))
}

#[test]
fn firewall_expert_can_analyze_network_intrusion() {
    let expert = FirewallExpert::new();
    let event = make_event(ThreatType::NetworkIntrusion, "10.0.0.1", "server");
    assert!(expert.can_analyze(&event));
}

#[test]
fn firewall_expert_cannot_analyze_virus() {
    let expert = FirewallExpert::new();
    let event = make_event(ThreatType::Virus, "source", "target");
    assert!(!expert.can_analyze(&event));
}

#[test]
fn firewall_expert_returns_log_for_unmatched_traffic() {
    let expert = FirewallExpert::new();
    let event = make_event(ThreatType::NetworkIntrusion, "unknown", "unknown");
    let verdict = expert.analyze(&event).unwrap();
    assert_eq!(verdict.action, ProtectionAction::Log);
}

#[test]
fn firewall_expert_type_is_firewall() {
    let expert = FirewallExpert::new();
    assert_eq!(expert.expert_type(), ExpertType::Firewall);
}
