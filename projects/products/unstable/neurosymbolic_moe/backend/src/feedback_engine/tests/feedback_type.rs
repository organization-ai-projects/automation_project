use crate::feedback_engine::FeedbackType;

#[test]
fn feedback_type_variants_are_constructible() {
    let positive = FeedbackType::Positive;
    let negative = FeedbackType::Negative;
    let correction = FeedbackType::Correction;
    let suggestion = FeedbackType::Suggestion;
    assert!(matches!(positive, FeedbackType::Positive));
    assert!(matches!(negative, FeedbackType::Negative));
    assert!(matches!(correction, FeedbackType::Correction));
    assert!(matches!(suggestion, FeedbackType::Suggestion));
}
