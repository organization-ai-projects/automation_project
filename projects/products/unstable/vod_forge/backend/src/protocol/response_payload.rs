// projects/products/unstable/vod_forge/backend/src/protocol/response_payload.rs
use serde::{Deserialize, Serialize};

use crate::protocol::TitleView;

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
    RecommendData {
        recommended: Vec<String>,
    },
    AnalyticsReport {
        total_watch_ticks: u64,
        completion_rate_pct: f32,
        episodes_watched: usize,
    },
}
