use crate::app::app_state::AppState;

pub struct RenderScreen;

impl RenderScreen {
    pub fn render(state: &AppState) {
        println!("=== Render Screen ===");
        if let Some(ref err) = state.last_error {
            println!("Error: {err}");
        } else {
            println!("Docs rendered.");
        }
    }
}
