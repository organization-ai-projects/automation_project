use crate::app::app_state::AppState;
use crate::widgets::log_widget::LogWidget;

pub struct RunScreen;

impl RunScreen {
    pub fn render(state: &AppState) {
        println!("== RUN ==");
        if let Some(error) = &state.last_error {
            LogWidget::render(&[format!("error: {error}")]);
        } else {
            println!("run ok");
        }
    }
}
