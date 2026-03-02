// projects/products/unstable/code_forge_engine/ui/src/app/app_state.rs
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Idle,
    ContractLoaded { path: String },
    Validating,
    Previewing,
    Generating,
    Done,
    Failed { reason: String },
}

impl Default for AppState {
    fn default() -> Self {
        Self::Idle
    }
}
