use crate::app::app_state::AppState;
use crate::widgets::log_widget::LogWidget;

pub struct BattleScreen;

impl BattleScreen {
    pub fn render(state: &AppState) {
        println!("== BATTLE ==");
        if let Some(ref json) = state.battle_json {
            LogWidget::render(&[format!("battle={json}")]);
        }
        if let Some(error) = &state.last_error {
            LogWidget::render(&[format!("error: {error}")]);
        }
    }
}
