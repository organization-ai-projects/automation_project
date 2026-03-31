use crate::journal::InvalidationRule;

#[test]
fn new_creates_untriggered_rule() {
    let rule = InvalidationRule::new("R1", "Revenue decline check", "revenue_yoy < -0.2");
    assert!(!rule.triggered);
}

#[test]
fn trigger_sets_flag() {
    let mut rule = InvalidationRule::new("R1", "Test rule", "condition");
    rule.trigger();
    assert!(rule.triggered);
}

#[test]
fn serialization_roundtrip() {
    let rule = InvalidationRule::new("R2", "Margin check", "gross_margin < 0.3");
    let json = common_json::to_json_string(&rule).unwrap();
    let restored: InvalidationRule = common_json::from_str(&json).unwrap();
    assert_eq!(rule, restored);
}
