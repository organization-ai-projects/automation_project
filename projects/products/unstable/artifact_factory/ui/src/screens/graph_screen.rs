use crate::app::app_state::AppState;

pub struct GraphScreen;

impl GraphScreen {
    pub fn render(state: &AppState) {
        println!("=== Graph Screen ===");
        println!("Inputs: {}", state.input_paths.len());
        if let Some(ref err) = state.last_error {
            println!("Error: {err}");
        }
    }
}
