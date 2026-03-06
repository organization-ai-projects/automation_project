// projects/products/unstable/colony_manager/ui/src/app/app_state.rs
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AppState {
    pub running: bool,
    pub replaying: bool,
}
