use crate::app::analytics_view::AnalyticsView;
use crate::app::catalog_entry::CatalogEntry;
use crate::app::playback_view::PlaybackView;
use crate::diagnostics::UiError;
use crate::transport::IpcClient;
use crate::transport::ipc_response::IpcResponse;
use crate::transport::request_payload::RequestPayload;
use crate::transport::response_payload::ResponsePayload;

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
        let resp: IpcResponse =
            self.ipc
                .send_request(writer, reader, &RequestPayload::CatalogList)?;
        Self::validate_response_id(resp.id)?;
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
        let req = RequestPayload::PlaybackStart {
            profile: profile.to_string(),
            episode_id: episode_id.to_string(),
        };
        let resp: IpcResponse = self.ipc.send_request(writer, reader, &req)?;
        Self::validate_response_id(resp.id)?;
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
        let req = RequestPayload::PlaybackStep {
            session_id: session_id.to_string(),
            steps,
        };
        let resp: IpcResponse = self.ipc.send_request(writer, reader, &req)?;
        Self::validate_response_id(resp.id)?;
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
        let req = RequestPayload::AnalyticsReport {
            profile: profile.to_string(),
        };
        let resp: IpcResponse = self.ipc.send_request(writer, reader, &req)?;
        Self::validate_response_id(resp.id)?;
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

    pub fn recommend<W: std::io::Write, R: std::io::BufRead>(
        &mut self,
        writer: &mut W,
        reader: &mut R,
        profile: &str,
        unwatched_only: bool,
    ) -> Result<Vec<String>, UiError> {
        let req = RequestPayload::Recommend {
            profile: profile.to_string(),
            unwatched_only,
        };
        let resp: IpcResponse = self.ipc.send_request(writer, reader, &req)?;
        Self::validate_response_id(resp.id)?;
        match resp.payload {
            ResponsePayload::RecommendData { recommended } => Ok(recommended),
            ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response".to_string())),
        }
    }

    fn validate_response_id(id: u64) -> Result<(), UiError> {
        if id == 0 {
            return Err(UiError::Ipc("invalid response id".to_string()));
        }
        Ok(())
    }
}
