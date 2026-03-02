#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Input,
    Graph,
    Render,
    Bundle,
}

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub current_screen: Screen,
    pub input_paths: Vec<String>,
    pub bundle_hash: Option<String>,
    pub bundle_manifest: Vec<String>,
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
