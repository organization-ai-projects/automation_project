use crate::sim::sim_engine::SimEngine;

#[test]
fn sim_engine_same_seed_produces_same_run_hash() {
    let mut first = SimEngine::with_defaults(99);
    let first_report = first.run(12);
    assert!(first_report.is_ok());

    let mut second = SimEngine::with_defaults(99);
    let second_report = second.run(12);
    assert!(second_report.is_ok());

    if let (Ok(left), Ok(right)) = (first_report, second_report) {
        assert_eq!(left.run_hash, right.run_hash);
    }
}
