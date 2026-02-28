use crate::app::app_state::{AnalyticsView, CatalogEntry, PlaybackView};
use crate::diagnostics::UiError;
use crate::transport::IpcClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(tag = "type")]
enum Request {
    CatalogList,
    PlaybackStart { profile: String, episode_id: String },
    PlaybackStep { session_id: String, steps: u32 },
    AnalyticsReport { profile: String },
}

#[derive(Deserialize)]
struct IpcResponse {
    #[allow(dead_code)]
    pub id: u64,
    pub payload: ResponsePayload,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ResponsePayload {
    Ok,
    Error {
        message: String,
    },
    CatalogData {
        titles: Vec<TitleViewRaw>,
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

#[derive(Deserialize)]
struct TitleViewRaw {
    pub id: String,
    pub name: String,
    pub year: u16,
}

pub struct Controller {
    pub ipc: IpcClient,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            ipc: IpcClient::new(),
        }
    }

    pub fn catalog_list<W: std::io::Write, R: std::io::BufRead>(
        &mut self,
        writer: &mut W,
        reader: &mut R,
    ) -> Result<Vec<CatalogEntry>, UiError> {
        let resp: IpcResponse = self
            .ipc
            .send_request(writer, reader, &Request::CatalogList)?;
        match resp.payload {
            ResponsePayload::CatalogData { titles } => Ok(titles
                .into_iter()
                .map(|t| CatalogEntry {
                    id: t.id,
                    name: t.name,
                    year: t.year,
                })
                .collect()),
            ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response".to_string())),
        }
    }

    pub fn playback_start<W: std::io::Write, R: std::io::BufRead>(
        &mut self,
        writer: &mut W,
        reader: &mut R,
        profile: &str,
        episode_id: &str,
    ) -> Result<PlaybackView, UiError> {
        let req = Request::PlaybackStart {
            profile: profile.to_string(),
            episode_id: episode_id.to_string(),
        };
        let resp: IpcResponse = self.ipc.send_request(writer, reader, &req)?;
        match resp.payload {
            ResponsePayload::PlaybackState {
                session_id,
                tick,
                progress_pct,
                done,
            } => Ok(PlaybackView {
                session_id,
                tick,
                progress_pct,
                done,
            }),
            ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response".to_string())),
        }
    }

    pub fn playback_step<W: std::io::Write, R: std::io::BufRead>(
        &mut self,
        writer: &mut W,
        reader: &mut R,
        session_id: &str,
        steps: u32,
    ) -> Result<PlaybackView, UiError> {
        let req = Request::PlaybackStep {
            session_id: session_id.to_string(),
            steps,
        };
        let resp: IpcResponse = self.ipc.send_request(writer, reader, &req)?;
        match resp.payload {
            ResponsePayload::PlaybackState {
                session_id,
                tick,
                progress_pct,
                done,
            } => Ok(PlaybackView {
                session_id,
                tick,
                progress_pct,
                done,
            }),
            ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response".to_string())),
        }
    }

    pub fn analytics_report<W: std::io::Write, R: std::io::BufRead>(
        &mut self,
        writer: &mut W,
        reader: &mut R,
        profile: &str,
    ) -> Result<AnalyticsView, UiError> {
        let req = Request::AnalyticsReport {
            profile: profile.to_string(),
        };
        let resp: IpcResponse = self.ipc.send_request(writer, reader, &req)?;
        match resp.payload {
            ResponsePayload::AnalyticsReport {
                total_watch_ticks,
                completion_rate_pct,
                episodes_watched,
            } => Ok(AnalyticsView {
                total_watch_ticks,
                completion_rate_pct,
                episodes_watched,
            }),
            ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response".to_string())),
        }
    }
}
