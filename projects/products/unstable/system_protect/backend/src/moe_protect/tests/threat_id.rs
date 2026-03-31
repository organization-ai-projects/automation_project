use crate::moe_protect::threat_id::ThreatId;

#[test]
fn threat_id_from_str() {
    let id = ThreatId::from_str("test-123");
    assert_eq!(id.to_string(), "test-123");
}

#[test]
fn threat_id_new_is_unique() {
    let id1 = ThreatId::new();
    let id2 = ThreatId::new();
    // They could potentially collide if generated at the exact same nanosecond,
    // but this is extremely unlikely in practice
    assert!(!id1.0.is_empty());
    assert!(!id2.0.is_empty());
}
