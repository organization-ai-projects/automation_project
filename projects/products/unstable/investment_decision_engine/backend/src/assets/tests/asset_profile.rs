use crate::assets::{AssetId, AssetProfile};

#[test]
fn new_creates_minimal_profile() {
    let id = AssetId::new("AAPL");
    let profile = AssetProfile::new(id.clone(), "Apple Inc.");
    assert_eq!(profile.id, id);
    assert_eq!(profile.name, "Apple Inc.");
    assert!(profile.sector.is_none());
}

#[test]
fn profile_serialization_roundtrip() {
    let id = AssetId::with_exchange("GOOG", "NASDAQ");
    let mut profile = AssetProfile::new(id, "Alphabet Inc.");
    profile.sector = Some("Technology".to_string());
    profile.market_cap_usd = Some(1_500_000_000_000.0);

    let json = common_json::to_json_string(&profile).unwrap();
    let restored: AssetProfile = common_json::from_str(&json).unwrap();
    assert_eq!(profile, restored);
}
