use crate::app::screen::Screen;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub current_screen: Screen,
    pub model_loaded: bool,
    pub snapshot_hash: Option<String>,
    pub snapshot_json: Option<String>,
    pub run_hash: Option<String>,
    pub last_report: Option<String>,
    pub replay_saved: bool,
    pub replay_data: Option<String>,
    pub last_error: Option<String>,
    pub running: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            running: true,
            ..Default::default()
        }
    }
}
