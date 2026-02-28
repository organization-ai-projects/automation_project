use crate::app::AppState;
use crate::widgets::ProgressWidget;

pub struct PlaybackScreen;

impl PlaybackScreen {
    pub fn render(state: &AppState) -> String {
        let mut out = String::from("=== Playback ===\n");
        match &state.playback {
            None => out.push_str("(no active session)\n"),
            Some(p) => {
                out.push_str(&format!("Session: {}\n", p.session_id));
                out.push_str(&format!("Tick: {}\n", p.tick));
                out.push_str(&ProgressWidget::render(p.progress_pct, 40));
                out.push('\n');
                if p.done {
                    out.push_str("DONE\n");
                }
            }
        }
        out
    }
}
