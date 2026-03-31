use crate::moe_protect::threat_type::ThreatType;

#[test]
fn threat_types_are_distinct() {
    assert_ne!(ThreatType::Virus, ThreatType::Malware);
    assert_ne!(ThreatType::NetworkIntrusion, ThreatType::DenialOfService);
}

#[test]
fn custom_threat_type_holds_value() {
    let custom = ThreatType::Custom("zero-day".to_string());
    assert_eq!(custom, ThreatType::Custom("zero-day".to_string()));
}
