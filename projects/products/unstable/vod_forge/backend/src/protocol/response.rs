use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleView {
    pub id: String,
    pub name: String,
    pub year: u16,
    pub episode_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcResponse {
    pub id: u64,
    pub payload: ResponsePayload,
}

#[derive(Debug, Serialize, Deserialize)]
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
        bundle_hash: String,
        chunk_count: usize,
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
