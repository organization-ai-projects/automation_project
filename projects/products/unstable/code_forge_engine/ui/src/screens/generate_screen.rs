use crate::app::app_state::AppState;

pub struct GenerateScreen;

impl GenerateScreen {
    pub fn render(state: &AppState) {
        let count = state.preview_files.len();
        println!("generation stage done for {count} files");
    }
}
