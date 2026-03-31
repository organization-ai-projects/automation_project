use crate::sentiment::market_fear_score::MarketFearScore;
use crate::sentiment::narrative_shift::{NarrativeDirection, NarrativeShift};
use crate::sentiment::sentiment_engine::{SentimentEngine, SentimentLabel};

#[test]
fn both_fearful_and_bearish_shift_is_bearish() {
    let fear = Some(MarketFearScore::from_score(15.0, "VIX"));
    let shifts = vec![NarrativeShift::new("2025-01-15", NarrativeDirection::BullishToBearish, "Panic", 0.9)];
    let summary = SentimentEngine::evaluate(fear, shifts);
    assert_eq!(summary.overall_sentiment, SentimentLabel::Bearish);
}

#[test]
fn only_fear_is_neutral() {
    let fear = Some(MarketFearScore::from_score(15.0, "VIX"));
    let shifts = vec![];
    let summary = SentimentEngine::evaluate(fear, shifts);
    assert_eq!(summary.overall_sentiment, SentimentLabel::Neutral);
}

#[test]
fn no_fear_no_shifts_is_bullish() {
    let summary = SentimentEngine::evaluate(None, vec![]);
    assert_eq!(summary.overall_sentiment, SentimentLabel::Bullish);
}
