use crate::protocol::serde_helpers::{deser_u16, deser_u32};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcRequest {
    pub id: u64,
    pub payload: RequestPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RequestPayload {
    CatalogAddTitle {
        title_id: String,
        name: String,
        #[serde(deserialize_with = "deser_u16")]
        year: u16,
    },
    CatalogAddEpisode {
        title_id: String,
        #[serde(deserialize_with = "deser_u32")]
        season: u32,
        #[serde(deserialize_with = "deser_u32")]
        episode: u32,
        name: String,
        #[serde(deserialize_with = "deser_u32")]
        duration_secs: u32,
    },
    CatalogList,
    PackageCreate {
        input_files: Vec<String>,
        out_bundle: String,
    },
    PackageVerify {
        bundle: String,
    },
    PlaybackStart {
        profile: String,
        episode_id: String,
    },
    PlaybackStep {
        session_id: String,
        #[serde(deserialize_with = "deser_u32")]
        steps: u32,
    },
    PlaybackStop {
        session_id: String,
    },
    AnalyticsReport {
        profile: String,
    },
}
