use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
    pub id: String,
    pub name: String,
    pub year: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackView {
    pub session_id: String,
    pub tick: u32,
    pub progress_pct: f32,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsView {
    pub total_watch_ticks: u64,
    pub completion_rate_pct: f32,
    pub episodes_watched: usize,
}

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub catalog_titles: Vec<CatalogEntry>,
    pub playback: Option<PlaybackView>,
    pub analytics: Option<AnalyticsView>,
    pub last_error: Option<String>,
}
