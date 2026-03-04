use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::screen::Screen;

pub struct Reducer;

impl Reducer {
    pub fn reduce(mut state: AppState, action: &Action) -> AppState {
        match action {
            Action::LoadContract(path) => {
                state.contract_path = Some(path.clone());
                state.screen = Screen::Contract;
            }
            Action::Validate => {
                state.screen = Screen::Contract;
            }
            Action::Preview => {
                state.screen = Screen::Preview;
            }
            Action::Generate { .. } => {
                state.screen = Screen::Generate;
            }
            Action::GetManifest => {
                state.screen = Screen::Report;
            }
            Action::Shutdown => {}
        }
        state
    }
}
