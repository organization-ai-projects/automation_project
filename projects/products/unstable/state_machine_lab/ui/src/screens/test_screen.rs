use crate::app::app_state::AppState;
use crate::widgets::log_widget::LogWidget;

pub struct TestScreen;

impl TestScreen {
    pub fn render(state: &AppState) {
        println!("== TEST ==");
        if let Some(report) = &state.test_report {
            LogWidget::render(&[format!("report: {report}")]);
        }
        if let Some(error) = &state.last_error {
            LogWidget::render(&[format!("error: {error}")]);
        }
    }
}
