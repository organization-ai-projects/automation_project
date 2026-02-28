use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub current_screen: Screen,
    pub generation: u32,
    pub best_fitness: f64,
    pub done: bool,
    pub last_manifest: Option<Value>,
    pub error: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Screen {
    #[default]
    Config,
    Running,
    Candidates,
    Report,
}
