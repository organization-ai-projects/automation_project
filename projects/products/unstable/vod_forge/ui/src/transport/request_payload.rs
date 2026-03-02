use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum RequestPayload {
    CatalogList,
    PlaybackStart {
        profile: String,
        episode_id: String,
    },
    PlaybackStep {
        session_id: String,
        steps: u32,
    },
    Recommend {
        profile: String,
        unwatched_only: bool,
    },
    AnalyticsReport {
        profile: String,
    },
}
