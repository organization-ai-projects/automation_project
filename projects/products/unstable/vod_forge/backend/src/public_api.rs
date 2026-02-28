use crate::analytics::{AnalyticsEvent, AnalyticsLog, AnalyticsReport};
use crate::catalog::CatalogStore;
use crate::diagnostics::BackendError;
use crate::packaging::{Packer, Unpacker};
use crate::playback::{PlaybackSession, ProfileId, SessionId};
use crate::protocol::response::TitleView;
use crate::protocol::{IpcRequest, IpcResponse, RequestPayload, ResponsePayload};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub struct PublicApi {
    catalog: CatalogStore,
    sessions: HashMap<String, PlaybackSession>,
    analytics: AnalyticsLog,
    session_counter: u64,
}

impl PublicApi {
    pub fn new() -> Self {
        PublicApi {
            catalog: CatalogStore::new(),
            sessions: HashMap::new(),
            analytics: AnalyticsLog::default(),
            session_counter: 0,
        }
    }

    pub fn handle_request(&mut self, req: IpcRequest) -> IpcResponse {
        let id = req.id;
        let payload = self.dispatch(req.payload);
        IpcResponse { id, payload }
    }

    fn dispatch(&mut self, payload: RequestPayload) -> ResponsePayload {
        match payload {
            RequestPayload::CatalogAddTitle {
                title_id,
                name,
                year,
            } => match self.catalog.add_title(&title_id, &name, year) {
                Ok(()) => ResponsePayload::Ok,
                Err(e) => ResponsePayload::Error {
                    message: e.to_string(),
                },
            },
            RequestPayload::CatalogAddEpisode {
                title_id,
                season,
                episode,
                name,
                duration_secs,
            } => {
                match self
                    .catalog
                    .add_episode(&title_id, season, episode, &name, duration_secs)
                {
                    Ok(()) => ResponsePayload::Ok,
                    Err(e) => ResponsePayload::Error {
                        message: e.to_string(),
                    },
                }
            }
            RequestPayload::CatalogList => {
                let titles: Vec<TitleView> = self
                    .catalog
                    .list()
                    .iter()
                    .map(|t| {
                        let episode_count: usize =
                            t.seasons.values().map(|s| s.episodes.len()).sum();
                        TitleView {
                            id: t.id.0.clone(),
                            name: t.name.clone(),
                            year: t.year,
                            episode_count,
                        }
                    })
                    .collect();
                ResponsePayload::CatalogData { titles }
            }
            RequestPayload::PackageCreate {
                input_files,
                out_bundle,
            } => {
                let loaded: Result<Vec<(String, Vec<u8>)>, BackendError> = input_files
                    .iter()
                    .map(|path| {
                        std::fs::read(path)
                            .map(|bytes| (path.clone(), bytes))
                            .map_err(|e| BackendError::Io(e.to_string()))
                    })
                    .collect();
                match loaded {
                    Err(e) => ResponsePayload::Error {
                        message: e.to_string(),
                    },
                    Ok(files) => {
                        // Use a deterministic bundle_id derived from sorted input filenames
                        let mut sorted_names: Vec<&str> =
                            files.iter().map(|(n, _)| n.as_str()).collect();
                        sorted_names.sort();
                        let bundle_id = sorted_names.join("|");
                        match Packer::pack(files, &bundle_id) {
                            Err(e) => ResponsePayload::Error {
                                message: e.to_string(),
                            },
                            Ok((manifest, bundle_bytes)) => {
                                let chunk_count = manifest.chunks.len();
                                if let Err(e) = std::fs::write(&out_bundle, &bundle_bytes) {
                                    return ResponsePayload::Error {
                                        message: e.to_string(),
                                    };
                                }
                                let mut hasher = Sha256::new();
                                hasher.update(&bundle_bytes);
                                let bundle_hash = hex::encode(hasher.finalize());
                                ResponsePayload::PackageResult {
                                    bundle_hash,
                                    chunk_count,
                                }
                            }
                        }
                    }
                }
            }
            RequestPayload::PackageVerify { bundle } => match std::fs::read(&bundle) {
                Err(e) => ResponsePayload::Error {
                    message: e.to_string(),
                },
                Ok(bytes) => match Unpacker::unpack(&bytes) {
                    Err(e) => ResponsePayload::Error {
                        message: e.to_string(),
                    },
                    Ok((manifest, _)) => {
                        let chunk_count = manifest.chunks.len();
                        let mut hasher = Sha256::new();
                        hasher.update(&bytes);
                        let bundle_hash = hex::encode(hasher.finalize());
                        ResponsePayload::PackageResult {
                            bundle_hash,
                            chunk_count,
                        }
                    }
                },
            },
            RequestPayload::PlaybackStart {
                profile,
                episode_id,
            } => {
                self.session_counter += 1;
                let session_id = SessionId::new(&episode_id, self.session_counter);
                // duration_ticks: use duration_secs from catalog if found, else 100
                let duration_ticks = self
                    .catalog
                    .catalog
                    .titles
                    .values()
                    .flat_map(|t| t.seasons.values())
                    .flat_map(|s| s.episodes.values())
                    .find(|e| e.id == episode_id)
                    .map(|e| e.duration_secs)
                    .unwrap_or(100);
                let session = PlaybackSession {
                    id: session_id.clone(),
                    profile: ProfileId::from(profile.as_str()),
                    episode_id: episode_id.clone(),
                    tick: 0,
                    duration_ticks,
                };
                let sid_str = session_id.0.clone();
                self.sessions.insert(sid_str.clone(), session);
                ResponsePayload::PlaybackState {
                    session_id: sid_str,
                    tick: 0,
                    progress_pct: 0.0,
                    done: false,
                }
            }
            RequestPayload::PlaybackStep { session_id, steps } => {
                match self.sessions.get_mut(&session_id) {
                    None => ResponsePayload::Error {
                        message: format!("session {} not found", session_id),
                    },
                    Some(session) => {
                        session.step(steps);
                        let tick = session.tick;
                        let progress_pct = session.progress_pct();
                        let done = session.is_done();
                        if done {
                            let event = AnalyticsEvent {
                                profile_id: session.profile.clone(),
                                episode_id: session.episode_id.clone(),
                                ticks_watched: tick,
                                completed: true,
                                at_tick: tick,
                            };
                            self.analytics.append(event);
                        }
                        ResponsePayload::PlaybackState {
                            session_id,
                            tick,
                            progress_pct,
                            done,
                        }
                    }
                }
            }
            RequestPayload::PlaybackStop { session_id } => {
                match self.sessions.remove(&session_id) {
                    None => ResponsePayload::Error {
                        message: format!("session {} not found", session_id),
                    },
                    Some(session) => {
                        let event = AnalyticsEvent {
                            profile_id: session.profile.clone(),
                            episode_id: session.episode_id.clone(),
                            ticks_watched: session.tick,
                            completed: session.is_done(),
                            at_tick: session.tick,
                        };
                        self.analytics.append(event);
                        ResponsePayload::Ok
                    }
                }
            }
            RequestPayload::AnalyticsReport { profile } => {
                let report = AnalyticsReport::from_log(&self.analytics, &profile);
                ResponsePayload::AnalyticsReport {
                    total_watch_ticks: report.total_watch_ticks,
                    completion_rate_pct: report.completion_rate_pct,
                    episodes_watched: report.episodes_watched,
                }
            }
        }
    }
}
