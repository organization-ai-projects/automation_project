use crate::app::app_state::AppState;

pub struct InputScreen;

impl InputScreen {
    pub fn render(state: &AppState) {
        println!("=== Input Screen ===");
        if state.input_paths.is_empty() {
            println!("No inputs loaded.");
        } else {
            for path in &state.input_paths {
                println!("  - {path}");
            }
        }
    }
}
