use crate::moe_protect::threat_level::ThreatLevel;

#[test]
fn threat_levels_are_ordered() {
    assert!(ThreatLevel::Info < ThreatLevel::Low);
    assert!(ThreatLevel::Low < ThreatLevel::Medium);
    assert!(ThreatLevel::Medium < ThreatLevel::High);
    assert!(ThreatLevel::High < ThreatLevel::Critical);
}
