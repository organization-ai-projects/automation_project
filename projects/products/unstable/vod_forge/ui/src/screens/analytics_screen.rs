use crate::app::AppState;

pub struct AnalyticsScreen;

impl AnalyticsScreen {
    pub fn render(state: &AppState) -> String {
        let mut out = String::from("=== Analytics ===\n");
        match &state.analytics {
            None => out.push_str("(no analytics)\n"),
            Some(a) => {
                out.push_str(&format!("Total ticks: {}\n", a.total_watch_ticks));
                out.push_str(&format!("Completion: {:.1}%\n", a.completion_rate_pct));
                out.push_str(&format!("Episodes watched: {}\n", a.episodes_watched));
            }
        }
        out
    }
}
