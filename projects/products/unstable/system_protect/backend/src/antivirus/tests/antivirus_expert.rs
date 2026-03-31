use crate::antivirus::antivirus_expert::AntivirusExpert;
use crate::moe_protect::expert::ProtectionExpert;
use crate::moe_protect::expert_type::ExpertType;
use crate::moe_protect::protection_action::ProtectionAction;
use crate::moe_protect::threat_event::ThreatEvent;
use crate::moe_protect::threat_id::ThreatId;
use crate::moe_protect::threat_level::ThreatLevel;
use crate::moe_protect::threat_type::ThreatType;

fn make_event(threat_type: ThreatType, payload: &str) -> ThreatEvent {
    ThreatEvent::new(threat_type, ThreatLevel::High, "src", "dst", payload)
        .with_id(ThreatId::from_str("test-threat-001"))
}

#[test]
fn antivirus_expert_can_analyze_virus_threats() {
    let expert = AntivirusExpert::new();
    let event = make_event(ThreatType::Virus, "some payload");
    assert!(expert.can_analyze(&event));
}

#[test]
fn antivirus_expert_cannot_analyze_network_intrusion() {
    let expert = AntivirusExpert::new();
    let event = make_event(ThreatType::NetworkIntrusion, "some payload");
    assert!(!expert.can_analyze(&event));
}

#[test]
fn antivirus_expert_returns_allow_for_clean_payload() {
    let expert = AntivirusExpert::new();
    let event = make_event(ThreatType::Virus, "normal file content");
    let verdict = expert.analyze(&event).unwrap();
    assert_eq!(verdict.action, ProtectionAction::Allow);
}

#[test]
fn antivirus_expert_returns_quarantine_for_ransomware_signature() {
    let expert = AntivirusExpert::new();
    let event = make_event(ThreatType::Ransomware, "encrypt_all_files");
    let verdict = expert.analyze(&event).unwrap();
    assert_eq!(verdict.action, ProtectionAction::Quarantine);
    assert!(verdict.confidence > 0.9);
}

#[test]
fn antivirus_expert_type_is_antivirus() {
    let expert = AntivirusExpert::new();
    assert_eq!(expert.expert_type(), ExpertType::Antivirus);
}
