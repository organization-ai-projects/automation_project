#![allow(dead_code)]
use crate::app::action::Action;
use crate::app::app_state::AppState;

pub struct Reducer;

impl Reducer {
    pub fn reduce(state: &AppState, action: &Action) -> AppState {
        let mut next = state.clone();
        match action {
            Action::SelectPack(pack) => {
                next.pack_kind = Some(pack.clone());
            }
            Action::SetSeed(seed) => {
                next.seed = *seed;
            }
            Action::SetTicks(ticks) => {
                next.ticks = *ticks;
            }
            Action::StartRun => {}
            Action::Shutdown => {}
        }
        next
    }
}
