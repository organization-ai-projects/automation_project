use crate::sentiment::market_fear_score::{FearLevel, MarketFearScore};

#[test]
fn extreme_fear_below_20() {
    let score = MarketFearScore::from_score(15.0, "VIX");
    assert_eq!(score.label, FearLevel::ExtremeFear);
    assert!(score.is_fearful());
}

#[test]
fn greed_above_60() {
    let score = MarketFearScore::from_score(75.0, "CNN");
    assert_eq!(score.label, FearLevel::Greed);
    assert!(!score.is_fearful());
}

#[test]
fn serialization_roundtrip() {
    let score = MarketFearScore::from_score(50.0, "Custom");
    let json = common_json::to_json_string(&score).unwrap();
    let restored: MarketFearScore = common_json::from_str(&json).unwrap();
    assert_eq!(score, restored);
}
