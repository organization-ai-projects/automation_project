// projects/products/unstable/city_builder/ui/src/widgets/status_widget.rs
use crate::app::app_state::AppState;

pub struct StatusWidget;

impl StatusWidget {
    pub fn render(state: &AppState) -> String {
        let command = state
            .last_command
            .as_ref()
            .map_or("none", std::string::String::as_str);
        let code = state.last_exit_code.unwrap_or(-1);
        format!(
            "backend={} command={} exit_code={}",
            state.backend_bin, command, code
        )
    }
}
