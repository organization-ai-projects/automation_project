use crate::neurosymbolic::InsightKind;
use crate::neurosymbolic::NeurosymbolicEngine;

#[test]
fn neurosymbolic_engine_creates_successfully() {
    let engine = NeurosymbolicEngine::new();
    assert!(engine.is_ok());
}

#[test]
fn neurosymbolic_analyze_returns_insights() {
    let mut engine = NeurosymbolicEngine::new().unwrap();
    let insights = engine.analyze("fn main() { let x = 1; }");
    assert!(insights.is_ok());
    let insights = insights.unwrap();
    assert!(!insights.is_empty());
}

#[test]
fn insight_kind_display() {
    assert_eq!(InsightKind::Suggestion.to_string(), "suggestion");
    assert_eq!(InsightKind::Refactoring.to_string(), "refactoring");
    assert_eq!(
        InsightKind::PatternDetection.to_string(),
        "pattern_detection"
    );
    assert_eq!(
        InsightKind::ComplexityWarning.to_string(),
        "complexity_warning"
    );
}
