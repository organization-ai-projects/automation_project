use crate::diagnostics::EngineError;

#[test]
fn error_display_input() {
    let err = EngineError::Input("missing field".to_string());
    assert_eq!(format!("{err}"), "input error: missing field");
}

#[test]
fn error_display_feature_disabled() {
    let err = EngineError::FeatureDisabled("recommendation_output".to_string());
    assert_eq!(format!("{err}"), "feature disabled: recommendation_output");
}

#[test]
fn error_variants_are_distinct() {
    let input = format!("{}", EngineError::Input("a".to_string()));
    let parse = format!("{}", EngineError::Parse("a".to_string()));
    let io = format!("{}", EngineError::Io("a".to_string()));
    assert_ne!(input, parse);
    assert_ne!(parse, io);
}
