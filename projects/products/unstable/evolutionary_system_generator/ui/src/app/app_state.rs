// projects/products/unstable/evolutionary_system_generator/ui/src/app/app_state.rs
use crate::app::screen::Screen;
use common_json::Json;

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub current_screen: Screen,
    pub generation: u32,
    pub best_fitness: f64,
    pub done: bool,
    pub last_manifest: Option<Json>,
    pub error: Option<String>,
}
