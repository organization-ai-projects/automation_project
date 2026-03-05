// projects/products/unstable/colony_manager/ui/src/app/tests/reducer.rs
#[cfg(test)]
#[test]
fn reducer_is_deterministic() {
    use crate::app::{action::Action, app_state::AppState};

    let mut s1 = AppState::default();
    let mut s2 = AppState::default();
    let actions = [
        Action::RunRequested,
        Action::RunCompleted,
        Action::ReplayRequested,
        Action::ReplayCompleted,
    ];
    for action in &actions {
        use crate::app::reducer::Reducer;

        Reducer::apply(&mut s1, action);
        Reducer::apply(&mut s2, action);
    }
    assert_eq!(s1, s2);
}
