use crate::decision::decision_confidence::{ConfidenceLabel, DecisionConfidence};

#[test]
fn high_score_is_very_high() {
    let dc = DecisionConfidence::from_score(0.95);
    assert_eq!(dc.label, ConfidenceLabel::VeryHigh);
}

#[test]
fn low_score_is_very_low() {
    let dc = DecisionConfidence::from_score(0.1);
    assert_eq!(dc.label, ConfidenceLabel::VeryLow);
}

#[test]
fn moderate_score() {
    let dc = DecisionConfidence::from_score(0.55);
    assert_eq!(dc.label, ConfidenceLabel::Moderate);
}
