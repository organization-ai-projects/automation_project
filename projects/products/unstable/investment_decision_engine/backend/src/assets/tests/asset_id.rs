use crate::assets::AssetId;

#[test]
fn new_creates_asset_without_exchange() {
    let id = AssetId::new("AAPL");
    assert_eq!(id.ticker, "AAPL");
    assert!(id.exchange.is_none());
}

#[test]
fn with_exchange_creates_full_id() {
    let id = AssetId::with_exchange("AAPL", "NASDAQ");
    assert_eq!(id.ticker, "AAPL");
    assert_eq!(id.exchange.as_deref(), Some("NASDAQ"));
}

#[test]
fn canonical_key_without_exchange() {
    let id = AssetId::new("MSFT");
    assert_eq!(id.canonical_key(), "MSFT");
}

#[test]
fn canonical_key_with_exchange() {
    let id = AssetId::with_exchange("MSFT", "NASDAQ");
    assert_eq!(id.canonical_key(), "NASDAQ:MSFT");
}

#[test]
fn serialization_roundtrip() {
    let id = AssetId::with_exchange("TSLA", "NASDAQ");
    let json = common_json::to_json_string(&id).unwrap();
    let restored: AssetId = common_json::from_str(&json).unwrap();
    assert_eq!(id, restored);
}
