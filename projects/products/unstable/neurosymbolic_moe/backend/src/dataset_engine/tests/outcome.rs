use crate::dataset_engine::Outcome;

#[test]
fn outcome_variants_are_constructible() {
    let success = Outcome::Success;
    let failure = Outcome::Failure;
    let partial = Outcome::Partial;
    let unknown = Outcome::Unknown;
    assert!(matches!(success, Outcome::Success));
    assert!(matches!(failure, Outcome::Failure));
    assert!(matches!(partial, Outcome::Partial));
    assert!(matches!(unknown, Outcome::Unknown));
}
