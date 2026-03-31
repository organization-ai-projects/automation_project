use crate::components::run_controls::RunControls;

#[test]
fn controls_store_seed_and_ticks() {
    let c = RunControls::new(42, 100);
    assert_eq!(c.seed(), 42);
    assert_eq!(c.ticks(), 100);
}
