use crate::model::candidate::Candidate;

#[test]
fn candidate_new_sets_defaults() {
    let candidate = Candidate::new("a", "Alice", 70, 65, 80, 40);
    assert_eq!(candidate.id.to_string(), "a");
    assert_eq!(candidate.name, "Alice");
    assert_eq!(candidate.money, 1_000_000);
    assert_eq!(candidate.approval, 0.25);
}
