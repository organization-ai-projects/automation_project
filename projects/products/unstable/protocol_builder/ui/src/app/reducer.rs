// projects/products/unstable/protocol_builder/ui/src/app/reducer.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;

pub fn reduce(state: &mut AppState, action: Action) {
    state.apply(action);
}
