use crate::config::kernel_config::KernelConfig;
use crate::io::json_codec;
use crate::packs::pack_registry::PackRegistry;
use crate::protocol::message::Message;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::replay::replay_codec::ReplayCodec;
use crate::replay::replay_engine::ReplayEngine;
use crate::replay::replay_file::ReplayFile;
use crate::report::run_report::RunReport;
use crate::scenario::scenario::Scenario;
use crate::scenario::scenario_loader::ScenarioLoader;
use crate::scenario::scenario_validator::ScenarioValidator;
use crate::snapshot::state_snapshot::StateSnapshot;
use sha2::{Digest, Sha256};

pub struct RequestDispatcher {
    config: KernelConfig,
    registry: PackRegistry,
    shutdown: bool,
    loaded_scenario: Option<Scenario>,
    loaded_replay: Option<ReplayFile>,
    run_state: Option<RunState>,
}

struct RunState {
    world: crate::ecs::world::World,
    scenario_hash: String,
    report: RunReport,
    snapshot: StateSnapshot,
    event_log_checksum: u64,
}

impl RequestDispatcher {
    pub fn new() -> Self {
        Self {
            config: KernelConfig::default(),
            registry: PackRegistry::new(),
            shutdown: false,
            loaded_scenario: None,
            loaded_replay: None,
            run_state: None,
        }
    }

    pub fn dispatch(&mut self, line: &str) -> String {
        let msg: Message = match json_codec::decode(line) {
            Ok(m) => m,
            Err(e) => {
                let resp = Response::error(None, "PARSE_ERROR", &format!("{}", e), "");
                return json_codec::encode(&resp).unwrap_or_else(|_| "{}".to_string());
            }
        };

        let req = match json_codec::decode::<Request>(
            &json_codec::encode(&msg.payload).unwrap_or_default(),
        ) {
            Ok(r) => r,
            Err(e) => {
                let resp = Response::error(Some(msg.id), "INVALID_REQUEST", &format!("{}", e), "");
                return json_codec::encode(&resp).unwrap_or_else(|_| "{}".to_string());
            }
        };

        let response = self.handle_request(msg.id, req);
        json_codec::encode(&response).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn should_shutdown(&self) -> bool {
        self.shutdown
    }

    pub fn registry(&self) -> &PackRegistry {
        &self.registry
    }

    pub fn extract_snapshot(&self) -> Option<&StateSnapshot> {
        self.run_state.as_ref().map(|s| &s.snapshot)
    }

    pub fn extract_report(&self) -> Option<&RunReport> {
        self.run_state.as_ref().map(|s| &s.report)
    }

    fn handle_request(&mut self, id: u64, req: Request) -> Response {
        match req {
            Request::Ping => Response::ok(id),
            Request::ListPacks => {
                let packs = self.registry.list_packs();
                Response::packs(id, packs)
            }
            Request::LoadScenario { path } => self.load_scenario(id, &path),
            Request::ValidateScenario { path } => self.validate_scenario(id, &path),
            Request::Shutdown => {
                self.shutdown = true;
                Response::ok(id)
            }
            Request::NewRun {
                pack_kind,
                seed,
                ticks,
                turns,
                ticks_per_turn,
            } => self.start_run(
                id,
                pack_kind,
                seed.max(0.0) as u64,
                ticks.max(0.0) as u64,
                turns.max(0.0) as u64,
                ticks_per_turn.max(0.0) as u64,
            ),
            Request::GetReport => {
                if let Some(state) = &self.run_state {
                    Response::report(id, &state.report)
                } else {
                    Response::error(Some(id), "NO_RUN", "No run in progress", "")
                }
            }
            Request::GetSnapshot { at_tick, at_turn } => {
                if let Some(state) = &self.run_state {
                    Response::snapshot(
                        id,
                        &state.snapshot,
                        at_tick.max(0.0) as u64,
                        at_turn.max(0.0) as u64,
                    )
                } else {
                    Response::error(Some(id), "NO_RUN", "No run in progress", "")
                }
            }
            Request::Query { query } => self.query_state(id, &query),
            Request::SaveReplay { path } => self.save_replay(id, &path),
            Request::LoadReplay { path } => self.load_replay(id, &path),
            Request::ReplayToEnd => self.replay_to_end(id),
            Request::SubmitCommands { .. } | Request::Step { .. } | Request::RunToEnd => {
                Response::ok(id)
            }
        }
    }

    fn load_scenario(&mut self, id: u64, path: &str) -> Response {
        match ScenarioLoader::load_from_file(path).and_then(|scenario| {
            ScenarioValidator::validate(&scenario)?;
            Ok(scenario)
        }) {
            Ok(scenario) => {
                let scenario_hash = scenario.hash();
                self.loaded_scenario = Some(scenario);
                Response::ScenarioValidated { id, scenario_hash }
            }
            Err(error) => Response::error(Some(id), "INVALID_SCENARIO", &error.to_string(), ""),
        }
    }

    fn validate_scenario(&mut self, id: u64, path: &str) -> Response {
        match ScenarioLoader::load_from_file(path).and_then(|scenario| {
            ScenarioValidator::validate(&scenario)?;
            Ok(scenario.hash())
        }) {
            Ok(scenario_hash) => Response::ScenarioValidated { id, scenario_hash },
            Err(error) => Response::error(Some(id), "INVALID_SCENARIO", &error.to_string(), ""),
        }
    }

    fn query_state(&mut self, id: u64, query_text: &str) -> Response {
        use crate::inspect::query::Query;
        use crate::inspect::query_engine::QueryEngine;

        let state = match &self.run_state {
            Some(value) => value,
            None => return Response::error(Some(id), "NO_RUN", "No run in progress", ""),
        };

        let parsed_query = common_json::from_str::<Query>(query_text).unwrap_or(Query {
            kind: query_text.to_string(),
            filter: None,
        });
        let report = QueryEngine::execute(&state.world, &parsed_query);
        match common_json::to_string(&report) {
            Ok(report_json) => Response::QueryReport { id, report_json },
            Err(error) => Response::error(Some(id), "SERIALIZATION", &error.to_string(), ""),
        }
    }

    fn save_replay(&mut self, id: u64, path: &str) -> Response {
        let state = match &self.run_state {
            Some(value) => value,
            None => return Response::error(Some(id), "NO_RUN", "No run in progress", ""),
        };

        let replay_file = ReplayFile {
            pack_id: state.report.pack_kind.clone(),
            pack_kind: state.report.pack_kind.clone(),
            scenario_hash: state.scenario_hash.clone(),
            seed: state.report.seed,
            commands: Vec::new(),
            rng_draws: Vec::new(),
            event_log_checksum: state.event_log_checksum,
        };

        let encoded = match ReplayCodec::encode(&replay_file) {
            Ok(value) => value,
            Err(error) => {
                return Response::error(Some(id), "SERIALIZATION", &error.to_string(), "");
            }
        };
        if let Err(error) = std::fs::write(path, encoded) {
            return Response::error(Some(id), "IO_ERROR", &error.to_string(), "");
        }

        self.loaded_replay = Some(replay_file);
        Response::ok(id)
    }

    fn load_replay(&mut self, id: u64, path: &str) -> Response {
        let encoded = match std::fs::read_to_string(path) {
            Ok(value) => value,
            Err(error) => return Response::error(Some(id), "IO_ERROR", &error.to_string(), ""),
        };

        match ReplayCodec::decode(&encoded) {
            Ok(file) => {
                self.loaded_replay = Some(file);
                Response::ok(id)
            }
            Err(error) => Response::error(Some(id), "INVALID_REPLAY", &error.to_string(), ""),
        }
    }

    fn replay_to_end(&mut self, id: u64) -> Response {
        let replay = match &self.loaded_replay {
            Some(value) => value,
            None => return Response::error(Some(id), "NO_REPLAY", "No replay loaded", ""),
        };

        match ReplayEngine::replay(replay) {
            Ok(checksum) => Response::QueryReport {
                id,
                report_json: format!("{{\"replay_checksum\":{}}}", checksum),
            },
            Err(error) => Response::error(Some(id), "REPLAY_MISMATCH", &error.to_string(), ""),
        }
    }

    fn start_run(
        &mut self,
        id: u64,
        pack_kind: String,
        seed: u64,
        ticks: u64,
        turns: u64,
        ticks_per_turn: u64,
    ) -> Response {
        use crate::determinism::seed::Seed;
        use crate::ecs::world::World;
        use crate::events::event_log::EventLog;
        use crate::report::run_hash::RunHash;
        use crate::snapshot::snapshot_hash::SnapshotHash;
        use crate::time::logical_clock::LogicalClock;

        let seed_val = Seed::new(seed);
        let mut world = World::new();
        let mut event_log = EventLog::new();
        let mut clock = LogicalClock::new();
        let effective_turns = if turns == 0 {
            self.config.max_turns
        } else {
            turns
        };
        let effective_ticks_per_turn = if ticks_per_turn == 0 {
            self.config.default_ticks_per_turn
        } else {
            ticks_per_turn
        };
        if effective_turns == 0 || effective_ticks_per_turn == 0 {
            return Response::error(Some(id), "INVALID_CONFIG", "Invalid run timing", "");
        }

        let pack = match self.registry.get_pack(&pack_kind) {
            Some(p) => p,
            None => {
                return Response::error(
                    Some(id),
                    "UNKNOWN_PACK",
                    &format!("Unknown pack: {}", pack_kind),
                    "",
                );
            }
        };

        pack.initialize(&mut world, seed_val);

        let mut tick_index = 0;
        while tick_index < ticks {
            clock.advance_tick();
            pack.tick(&mut world, &clock, &mut event_log);
            tick_index += 1;
        }

        let snapshot = StateSnapshot::from_world(&world, &clock);
        let snapshot_hash = SnapshotHash::compute(&snapshot);
        let event_log_checksum = event_log.checksum();
        let run_hash = RunHash::compute(&event_log, &snapshot_hash);
        let report = RunReport::new(pack_kind, seed, ticks, run_hash, event_log.len());
        let scenario_hash = self
            .loaded_scenario
            .as_ref()
            .map(Scenario::hash)
            .unwrap_or_else(|| Self::fallback_scenario_hash(seed, ticks));

        self.run_state = Some(RunState {
            world: world.clone(),
            scenario_hash,
            report: report.clone(),
            snapshot,
            event_log_checksum,
        });
        Response::report(id, &report)
    }

    fn fallback_scenario_hash(seed: u64, ticks: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(seed.to_le_bytes());
        hasher.update(ticks.to_le_bytes());
        hex::encode(hasher.finalize())
    }
}
