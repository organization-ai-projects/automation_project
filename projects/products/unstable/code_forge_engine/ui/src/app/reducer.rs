// projects/products/unstable/code_forge_engine/ui/src/app/reducer.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;

pub struct Reducer;

impl Reducer {
    pub fn reduce(state: AppState, action: &Action) -> AppState {
        match action {
            Action::LoadContract(path) => AppState::ContractLoaded { path: path.clone() },
            Action::Validate => AppState::Validating,
            Action::Preview => AppState::Previewing,
            Action::Generate { .. } => AppState::Generating,
            Action::GetManifest | Action::Shutdown => state,
        }
    }
}
