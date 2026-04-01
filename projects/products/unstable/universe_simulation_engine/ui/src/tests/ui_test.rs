#[test]
fn default_state_sanity() {
    let state = crate::app::app_state::AppState::default();
    assert!(!state.running);
    assert_eq!(state.seed, 42);
    assert_eq!(state.ticks, 1000);
    assert_eq!(state.ticks_per_era, 50);
    assert!(state.gravity_enabled);
    assert!(state.electromagnetism_enabled);
    assert!(state.strong_nuclear_enabled);
    assert!(state.weak_nuclear_enabled);
    assert!(state.dark_matter_enabled);
    assert!(state.dark_energy_enabled);
    assert!(state.thermodynamics_enabled);
}
