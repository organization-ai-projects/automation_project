use crate::moe_protect::threat_event::ThreatEvent;
use crate::moe_protect::threat_id::ThreatId;
use crate::moe_protect::threat_level::ThreatLevel;
use crate::moe_protect::threat_type::ThreatType;

#[test]
fn threat_event_new_sets_fields() {
    let event = ThreatEvent::new(
        ThreatType::Virus,
        ThreatLevel::High,
        "source",
        "target",
        "payload",
    );
    assert_eq!(event.threat_type, ThreatType::Virus);
    assert_eq!(event.threat_level, ThreatLevel::High);
    assert_eq!(event.source, "source");
    assert_eq!(event.target, "target");
    assert_eq!(event.payload, "payload");
}

#[test]
fn threat_event_with_id_overrides_id() {
    let event = ThreatEvent::new(
        ThreatType::Malware,
        ThreatLevel::Medium,
        "src",
        "dst",
        "data",
    )
    .with_id(ThreatId::from_str("custom-id"));
    assert_eq!(event.id.to_string(), "custom-id");
}
