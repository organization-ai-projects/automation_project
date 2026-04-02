use crate::app::screen::Screen;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub current_screen: Screen,
    pub machine_loaded: bool,
    pub validated: bool,
    pub test_report: Option<String>,
    pub transcript: Option<String>,
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
