use crate::report::run_hash::RunHash;

#[test]
fn deterministic_hash() {
    let h1 = RunHash::compute(42, 100, 50, 5000);
    let h2 = RunHash::compute(42, 100, 50, 5000);
    assert_eq!(h1, h2);
}

#[test]
fn different_seed_different_hash() {
    let h1 = RunHash::compute(42, 100, 50, 5000);
    let h2 = RunHash::compute(43, 100, 50, 5000);
    assert_ne!(h1, h2);
}
