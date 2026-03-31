use crate::symbolic_engine::fact::Fact;

#[test]
fn fact_matches_exact_pattern() {
    let fact = Fact::new("threat", "is_type", "virus");
    assert!(fact.matches_pattern(Some("threat"), Some("is_type"), Some("virus")));
}

#[test]
fn fact_matches_partial_pattern() {
    let fact = Fact::new("threat", "is_type", "virus");
    assert!(fact.matches_pattern(Some("threat"), None, None));
}

#[test]
fn fact_does_not_match_wrong_value() {
    let fact = Fact::new("threat", "is_type", "virus");
    assert!(!fact.matches_pattern(
        Some("threat"),
        Some("is_type"),
        Some("malware")
    ));
}
