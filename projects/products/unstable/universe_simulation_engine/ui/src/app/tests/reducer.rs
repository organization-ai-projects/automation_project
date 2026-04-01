#[cfg(test)]
#[test]
fn reducer_is_deterministic() {
    use crate::app::{action::Action, app_state::AppState};
    use crate::app::reducer::Reducer;

    let mut s1 = AppState::default();
    let mut s2 = AppState::default();
    let actions = [
        Action::RunRequested,
        Action::ToggleGravity,
        Action::ToggleElectromagnetism,
        Action::SetSeed(99),
        Action::SetTicks(500),
        Action::SetTicksPerEra(25),
        Action::RunCompleted,
    ];
    for action in &actions {
        Reducer::apply(&mut s1, action);
        Reducer::apply(&mut s2, action);
    }
    assert_eq!(s1, s2);
}

#[test]
fn toggle_gravity() {
    use crate::app::{action::Action, app_state::AppState};
    use crate::app::reducer::Reducer;

    let mut state = AppState::default();
    assert!(state.gravity_enabled);
    Reducer::apply(&mut state, &Action::ToggleGravity);
    assert!(!state.gravity_enabled);
    Reducer::apply(&mut state, &Action::ToggleGravity);
    assert!(state.gravity_enabled);
}

#[test]
fn set_seed() {
    use crate::app::{action::Action, app_state::AppState};
    use crate::app::reducer::Reducer;

    let mut state = AppState::default();
    Reducer::apply(&mut state, &Action::SetSeed(999));
    assert_eq!(state.seed, 999);
}

#[test]
fn run_lifecycle() {
    use crate::app::{action::Action, app_state::AppState};
    use crate::app::reducer::Reducer;

    let mut state = AppState::default();
    assert!(!state.running);
    Reducer::apply(&mut state, &Action::RunRequested);
    assert!(state.running);
    Reducer::apply(&mut state, &Action::RunCompleted);
    assert!(!state.running);
}
