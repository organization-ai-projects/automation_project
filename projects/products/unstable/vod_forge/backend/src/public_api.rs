use crate::analytics::{AnalyticsEvent, AnalyticsLog, AnalyticsReport};
use crate::catalogs::CatalogStore;
use crate::diagnostics::BackendError;
use crate::packaging::{Packer, Unpacker};
use crate::playback::{
    History, HistoryEntry, PlaybackSession, Profile, ProfileId, Progress, SessionId,
};
use crate::protocol::{IpcRequest, IpcResponse, RequestPayload, ResponsePayload, TitleView};
use crate::recommend::{RecommendReport, RecommendRule, Recommender};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub struct PublicApi {
    catalog: CatalogStore,
    sessions: HashMap<String, PlaybackSession>,
    profiles: HashMap<String, Profile>,
    history: History,
    analytics: AnalyticsLog,
    session_counter: u64,
}

impl PublicApi {
    pub fn new() -> Self {
        PublicApi {
            catalog: CatalogStore::new(),
            sessions: HashMap::new(),
            profiles: HashMap::new(),
            history: History::default(),
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
                Err(e) => Self::error_payload(e),
            },
            RequestPayload::CatalogAddEpisode {
                title_id,
                season,
                episode,
                name,
                duration_secs,
            } => match self
                .catalog
                .add_episode(&title_id, season, episode, &name, duration_secs)
            {
                Ok(()) => ResponsePayload::Ok,
                Err(e) => Self::error_payload(e),
            },
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
            } => self.package_create(&input_files, &out_bundle),
            RequestPayload::PackageVerify { bundle } => self.package_verify(&bundle),
            RequestPayload::PlaybackStart {
                profile,
                episode_id,
            } => self.playback_start(profile, episode_id),
            RequestPayload::PlaybackStep { session_id, steps } => {
                self.playback_step(session_id, steps)
            }
            RequestPayload::PlaybackStop { session_id } => self.playback_stop(session_id),
            RequestPayload::Recommend {
                profile,
                unwatched_only,
            } => self.recommend(profile, unwatched_only),
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

    fn package_create(&mut self, input_files: &[String], out_bundle: &str) -> ResponsePayload {
        let loaded: Result<Vec<(String, Vec<u8>)>, BackendError> = input_files
            .iter()
            .map(|path| {
                std::fs::read(path)
                    .map(|bytes| (path.clone(), bytes))
                    .map_err(|e| BackendError::Io(e.to_string()))
            })
            .collect();

        let files = match loaded {
            Ok(files) => files,
            Err(e) => return Self::error_payload(e),
        };

        let mut sorted_names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();
        sorted_names.sort();
        let bundle_id = sorted_names.join("|");

        let (manifest, bundle_bytes) = match Packer::pack(files, &bundle_id) {
            Ok(v) => v,
            Err(e) => return Self::error_payload(e),
        };

        let chunk_count = manifest.chunks.len();
        if let Err(e) = std::fs::write(out_bundle, &bundle_bytes) {
            return Self::error_payload(BackendError::Io(e.to_string()));
        }

        let mut hasher = Sha256::new();
        hasher.update(&bundle_bytes);
        let bundle_hash = hex::encode(hasher.finalize());
        ResponsePayload::PackageResult {
            bundle_hash,
            chunk_count,
        }
    }

    fn package_verify(&mut self, bundle: &str) -> ResponsePayload {
        let bytes = match std::fs::read(bundle) {
            Ok(bytes) => bytes,
            Err(e) => return Self::error_payload(BackendError::Io(e.to_string())),
        };

        let (manifest, _) = match Unpacker::unpack(&bytes) {
            Ok(v) => v,
            Err(e) => return Self::error_payload(e),
        };

        let chunk_count = manifest.chunks.len();
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let bundle_hash = hex::encode(hasher.finalize());
        ResponsePayload::PackageResult {
            bundle_hash,
            chunk_count,
        }
    }

    fn playback_start(&mut self, profile: String, episode_id: String) -> ResponsePayload {
        self.session_counter += 1;
        let session_id = SessionId::new(&episode_id, self.session_counter);
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

        let profile_id = ProfileId::from(profile.as_str());
        self.profiles
            .entry(profile.clone())
            .or_insert_with(|| Profile {
                id: profile_id.clone(),
                name: profile.clone(),
            });

        let session = PlaybackSession {
            id: session_id.clone(),
            profile: profile_id,
            episode_id: episode_id.clone(),
            tick: 0,
            duration_ticks,
        };
        let sid_str = session_id.0.clone();
        self.sessions.insert(sid_str.clone(), session);

        let progress = Progress {
            episode_id,
            tick: 0,
            duration_ticks,
        };

        ResponsePayload::PlaybackState {
            session_id: sid_str,
            tick: progress.tick,
            progress_pct: progress.progress_pct(),
            done: false,
        }
    }

    fn playback_step(&mut self, session_id: String, steps: u32) -> ResponsePayload {
        let Some(session) = self.sessions.get_mut(&session_id) else {
            return Self::error_payload(BackendError::Playback(format!(
                "session {} not found",
                session_id
            )));
        };

        session.step(steps);

        let done = session.is_done();
        let progress_pct = session.progress_pct();

        if done {
            let event = AnalyticsEvent {
                profile_id: session.profile.clone(),
                episode_id: session.episode_id.clone(),
                ticks_watched: session.tick,
                completed: true,
                at_tick: session.tick,
            };
            self.analytics.append(event);
            self.history.record(HistoryEntry {
                profile_id: session.profile.clone(),
                episode_id: session.episode_id.clone(),
                completed: true,
                ticks_watched: session.tick,
            });
        }

        ResponsePayload::PlaybackState {
            session_id,
            tick: session.tick,
            progress_pct,
            done,
        }
    }

    fn playback_stop(&mut self, session_id: String) -> ResponsePayload {
        let Some(session) = self.sessions.remove(&session_id) else {
            return Self::error_payload(BackendError::Playback(format!(
                "session {} not found",
                session_id
            )));
        };

        let event = AnalyticsEvent {
            profile_id: session.profile.clone(),
            episode_id: session.episode_id.clone(),
            ticks_watched: session.tick,
            completed: session.is_done(),
            at_tick: session.tick,
        };
        self.analytics.append(event);
        let completed = session.is_done();
        self.history.record(HistoryEntry {
            profile_id: session.profile,
            episode_id: session.episode_id,
            completed,
            ticks_watched: session.tick,
        });

        ResponsePayload::Ok
    }

    fn recommend(&mut self, profile: String, unwatched_only: bool) -> ResponsePayload {
        self.profiles
            .entry(profile.clone())
            .or_insert_with(|| Profile {
                id: ProfileId::from(profile.as_str()),
                name: profile.clone(),
            });

        let rule = RecommendRule {
            genre_match: false,
            unwatched_only,
        };
        let recommended =
            Recommender::recommend(&self.catalog.catalog, &self.history, &profile, &rule);

        let report = RecommendReport {
            profile_id: ProfileId::from(profile.as_str()),
            recommended,
        };

        ResponsePayload::RecommendData {
            recommended: report.recommended,
        }
    }

    fn error_payload(error: BackendError) -> ResponsePayload {
        ResponsePayload::Error {
            message: error.to_string(),
        }
    }
}
