use crate::transport::title_view::TitleView;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ResponsePayload {
    Ok,
    Error {
        message: String,
    },
    CatalogData {
        titles: Vec<TitleView>,
    },
    PackageResult {
        #[serde(default)]
        _ignored: Option<serde::de::IgnoredAny>,
    },
    RecommendData {
        recommended: Vec<String>,
    },
    PlaybackState {
        session_id: String,
        tick: u32,
        progress_pct: f32,
        done: bool,
    },
    AnalyticsReport {
        total_watch_ticks: u64,
        completion_rate_pct: f32,
        episodes_watched: usize,
    },
}
