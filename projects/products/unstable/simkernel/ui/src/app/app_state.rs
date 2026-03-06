// projects/products/unstable/simkernel/ui/src/app/app_state.rs
#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub pack_kind: Option<String>,
    pub seed: u64,
    pub ticks: u64,
    pub last_report: Option<String>,
    pub last_error: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}
