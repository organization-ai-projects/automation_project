use crate::commands::command::Command;
use crate::commands::command_id::CommandId;
use crate::commands::command_kind::CommandKind;
use crate::commands::command_validator::CommandValidator;
use crate::config::kernel_config::KernelConfig;
use crate::config::run_config::RunConfig;
use crate::determinism::rng_draw::RngDraw;
use crate::diagnostics::backend_error::BackendError;
use crate::ecs::component::Component;
use crate::ecs::component_id::ComponentId;
use crate::ecs::world::World;
use crate::events::event_id::EventId;
use crate::events::event_log::EventLog;
use crate::events::kernel_event::KernelEvent;
use crate::io::json_codec;
use crate::packs::pack::Pack;
use crate::packs::pack_registry::PackRegistry;
use crate::protocol::message::Message;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::replay::replay_codec::ReplayCodec;
use crate::replay::replay_engine::ReplayEngine;
use crate::replay::replay_file::ReplayFile;
use crate::report::report_codec::ReportCodec;
use crate::report::run_hash::RunHash;
use crate::report::run_report::RunReport;
use crate::report::tick_report::TickReport;
use crate::scenarios::scenario::Scenario;
use crate::scenarios::scenario_loader::ScenarioLoader;
use crate::scenarios::scenario_validator::ScenarioValidator;
use crate::schedule::system::System;
use crate::schedule::system_context::SystemContext;
use crate::schedule::system_id::SystemId;
use crate::schedule::system_scheduler::SystemScheduler;
use crate::schedule::system_stage::SystemStage;
use crate::snapshot::snapshot_hash::SnapshotHash;
use crate::snapshot::snapshot_id::SnapshotId;
use crate::snapshot::snapshot_store::SnapshotStore;
use crate::snapshot::state_snapshot::StateSnapshot;
use crate::time::logical_clock::LogicalClock;
use crate::time::phase::Phase;
use crate::time::tick::Tick;
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
    pack_kind: String,
    world: World,
    clock: LogicalClock,
    event_log: EventLog,
    scenario_hash: String,
    report: RunReport,
    report_json: String,
    snapshot: StateSnapshot,
    snapshot_store: SnapshotStore,
    event_log_checksum: u64,
    tick_reports: Vec<TickReport>,
    executed_ticks: u64,
}

struct HeartbeatSystem;

impl HeartbeatSystem {
    fn phase_name(phase: Phase) -> &'static str {
        match phase {
            Phase::PreTick => "PreTick",
            Phase::Tick => "Tick",
            Phase::PostTick => "PostTick",
            Phase::TurnBoundary => "TurnBoundary",
            Phase::Report => "Report",
        }
    }
}

impl System for HeartbeatSystem {
    fn id(&self) -> SystemId {
        SystemId::new(1)
    }

    fn stage(&self) -> SystemStage {
        SystemStage::Tick
    }

    fn run(&self, ctx: &mut SystemContext<'_>) {
        let transient = ctx.world.spawn();
        let marker = ComponentId::new(10_000);
        ctx.world
            .insert_component(transient, marker, Component::Flag(true));
        ctx.world.remove_component(transient, marker);
        ctx.world.despawn(transient);

        let mut payload = common_json::JsonMap::new();
        payload.insert(
            "tick".to_string(),
            common_json::Json::from(ctx.clock.tick.0),
        );
        payload.insert(
            "phase".to_string(),
            common_json::Json::from(Self::phase_name(Phase::Tick)),
        );
        let event = KernelEvent::new(
            EventId(ctx.clock.tick.0),
            ctx.clock.tick,
            "scheduler.heartbeat",
            common_json::Json::Object(payload),
        );
        ctx.emit(event);
    }
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
            Request::SubmitCommands { commands } => self.submit_commands(id, &commands),
            Request::Step { n_ticks } => self.advance_existing_run(id, n_ticks.max(0.0) as u64),
            Request::RunToEnd => {
                let remaining = self
                    .run_state
                    .as_ref()
                    .map(|state| self.config.max_ticks.saturating_sub(state.executed_ticks))
                    .unwrap_or(0);
                self.advance_existing_run(id, remaining)
            }
            Request::GetReport => {
                if let Some(state) = &self.run_state {
                    match ReportCodec::decode(&state.report_json) {
                        Ok(report) => Response::report(id, &report),
                        Err(_) => Response::report(id, &state.report),
                    }
                } else {
                    Response::error(Some(id), "NO_RUN", "No run in progress", "")
                }
            }
            Request::GetSnapshot { at_tick, at_turn } => {
                if let Some(state) = &self.run_state {
                    let requested_tick = at_tick.max(0.0) as u64;
                    let requested_turn = at_turn.max(0.0) as u64;
                    let snapshot_id = Self::snapshot_id_for(requested_tick, requested_turn);
                    let snapshot = state
                        .snapshot_store
                        .get(&snapshot_id)
                        .unwrap_or(&state.snapshot);
                    Response::snapshot(id, snapshot, requested_tick, requested_turn)
                } else {
                    Response::error(Some(id), "NO_RUN", "No run in progress", "")
                }
            }
            Request::Query { query } => self.query_state(id, &query),
            Request::SaveReplay { path } => self.save_replay(id, &path),
            Request::LoadReplay { path } => self.load_replay(id, &path),
            Request::ReplayToEnd => self.replay_to_end(id),
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

    fn submit_commands(&mut self, id: u64, commands: &[String]) -> Response {
        let state = match self.run_state.as_mut() {
            Some(value) => value,
            None => return Response::error(Some(id), "NO_RUN", "No run in progress", ""),
        };

        for (index, command) in commands.iter().enumerate() {
            let mut payload = common_json::JsonMap::new();
            payload.insert(
                "command".to_string(),
                common_json::Json::from(command.as_str()),
            );
            let synthetic_command = Command {
                id: CommandId(index as u64),
                kind: CommandKind::Custom("submitted".to_string()),
                at_tick: state.clock.tick,
                payload: common_json::Json::Object(payload.clone()),
            };
            if let Err(error) = CommandValidator::validate(&synthetic_command) {
                return Response::error(Some(id), "INVALID_COMMAND", &error.to_string(), "");
            }
            state.event_log.emit(
                state.clock.tick,
                "command.submitted",
                common_json::Json::Object(payload),
            );
        }
        Self::refresh_derived_state(state);
        Response::ok(id)
    }

    fn save_replay(&mut self, id: u64, path: &str) -> Response {
        let state = match &self.run_state {
            Some(value) => value,
            None => return Response::error(Some(id), "NO_RUN", "No run in progress", ""),
        };

        if state.event_log.is_empty() {
            return Response::error(Some(id), "NO_EVENTS", "Run has no events", "");
        }

        let commands: Vec<Command> = state
            .event_log
            .iter()
            .map(|event| Command {
                id: CommandId(event.id.0),
                kind: CommandKind::Custom(event.kind.clone()),
                at_tick: event.tick,
                payload: event.payload.clone(),
            })
            .collect();

        let rng_draws: Vec<RngDraw> = state
            .tick_reports
            .iter()
            .map(|tick_report| {
                RngDraw::new(
                    Tick(tick_report.tick),
                    "event_count",
                    tick_report.event_count as u64,
                )
            })
            .collect();

        let replay_file = ReplayFile {
            pack_id: state.pack_kind.clone(),
            pack_kind: state.pack_kind.clone(),
            scenario_hash: state.scenario_hash.clone(),
            seed: state.report.seed,
            commands,
            rng_draws,
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

    fn advance_existing_run(&mut self, id: u64, n_ticks: u64) -> Response {
        if n_ticks == 0 {
            return Response::ok(id);
        }

        let pack_kind = match self.run_state.as_ref() {
            Some(state) => state.pack_kind.clone(),
            None => return Response::error(Some(id), "NO_RUN", "No run in progress", ""),
        };
        let pack = match self.registry.get_pack(&pack_kind) {
            Some(value) => value,
            None => return Response::error(Some(id), "UNKNOWN_PACK", "Unknown active pack", ""),
        };

        let state = match self.run_state.as_mut() {
            Some(value) => value,
            None => return Response::error(Some(id), "NO_RUN", "No run in progress", ""),
        };
        Self::advance_run_state(state, pack, n_ticks);

        Response::report(id, &state.report)
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

        let run_config = RunConfig {
            pack_kind: pack_kind.clone(),
            seed,
            ticks,
            turns,
            ticks_per_turn,
        };
        let seed_val = Seed::new(seed);
        let mut world = World::new();
        let event_log = EventLog::new();
        let mut clock = LogicalClock::new();
        let effective_turns = if run_config.turns == 0 {
            self.config.max_turns
        } else {
            run_config.turns
        };
        let effective_ticks_per_turn = if run_config.ticks_per_turn == 0 {
            self.config.default_ticks_per_turn
        } else {
            run_config.ticks_per_turn
        };
        if effective_turns == 0 || effective_ticks_per_turn == 0 {
            return Response::error(Some(id), "INVALID_CONFIG", "Invalid run timing", "");
        }

        clock.ticks_per_turn = effective_ticks_per_turn;

        let pack = match self.registry.get_pack(&run_config.pack_kind) {
            Some(p) => p,
            None => {
                return Response::error(
                    Some(id),
                    "UNKNOWN_PACK",
                    &BackendError::Internal(format!("Unknown pack: {}", run_config.pack_kind))
                        .to_string(),
                    "",
                );
            }
        };

        let pack_identity = format!("{}:{}", pack.id().0, pack.name());
        pack.initialize(&mut world, seed_val);

        let scenario_hash = self
            .loaded_scenario
            .as_ref()
            .map(Scenario::hash)
            .unwrap_or_else(|| {
                Self::fallback_scenario_hash(run_config.seed, run_config.ticks, &pack_identity)
            });

        let initial_snapshot = StateSnapshot::from_world(&world, &clock);
        let mut snapshot_store = SnapshotStore::new();
        snapshot_store.store(
            Self::snapshot_id_for(initial_snapshot.tick, initial_snapshot.turn),
            initial_snapshot.clone(),
        );

        let mut state = RunState {
            pack_kind: run_config.pack_kind.clone(),
            world,
            clock,
            event_log,
            scenario_hash,
            report: RunReport::new(run_config.pack_kind, run_config.seed, 0, String::new(), 0),
            report_json: String::new(),
            snapshot: initial_snapshot,
            snapshot_store,
            event_log_checksum: 0,
            tick_reports: Vec::new(),
            executed_ticks: 0,
        };

        Self::refresh_derived_state(&mut state);
        if run_config.ticks > 0 {
            Self::advance_run_state(&mut state, pack, run_config.ticks);
        }

        let report = state.report.clone();
        self.run_state = Some(state);
        Response::report(id, &report)
    }

    fn build_scheduler() -> SystemScheduler {
        let mut scheduler = SystemScheduler::new();
        scheduler.register(Box::new(HeartbeatSystem));
        scheduler
    }

    fn advance_run_state(state: &mut RunState, pack: &dyn Pack, n_ticks: u64) {
        let scheduler = Self::build_scheduler();

        for _ in 0..n_ticks {
            let previous_turn = state.clock.turn.0;
            state.clock.advance_tick();
            pack.tick(&mut state.world, &state.clock, &mut state.event_log);
            scheduler.run_tick(&mut state.world, &state.clock, &mut state.event_log);
            if state.clock.turn.0 != previous_turn {
                scheduler.run_stage(
                    SystemStage::TurnBoundary,
                    &mut state.world,
                    &state.clock,
                    &mut state.event_log,
                );
            }

            state.executed_ticks += 1;
            let snapshot = StateSnapshot::from_world(&state.world, &state.clock);
            let snapshot_id = Self::snapshot_id_for(snapshot.tick, snapshot.turn);
            state.snapshot_store.store(snapshot_id, snapshot.clone());
            state.snapshot = snapshot.clone();
            state.tick_reports.push(TickReport {
                tick: snapshot.tick,
                event_count: state.event_log.len(),
                snapshot_hash: snapshot.hash,
            });
        }

        scheduler.run_stage(
            SystemStage::Report,
            &mut state.world,
            &state.clock,
            &mut state.event_log,
        );
        Self::refresh_derived_state(state);
    }

    fn refresh_derived_state(state: &mut RunState) {
        let snapshot_hash = SnapshotHash::compute(&state.snapshot);
        state.event_log_checksum = state.event_log.checksum();
        let run_hash = RunHash::compute(&state.event_log, &snapshot_hash);
        state.report = RunReport::new(
            state.pack_kind.clone(),
            state.report.seed,
            state.executed_ticks,
            run_hash,
            state.event_log.len(),
        );
        state.report_json = ReportCodec::encode_canonical(&state.report).unwrap_or_default();
    }

    fn snapshot_id_for(tick: u64, turn: u64) -> SnapshotId {
        SnapshotId(format!("tick:{}:turn:{}", tick, turn))
    }

    fn fallback_scenario_hash(seed: u64, ticks: u64, pack_identity: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(seed.to_le_bytes());
        hasher.update(ticks.to_le_bytes());
        hasher.update(pack_identity.as_bytes());
        hex::encode(hasher.finalize())
    }
}
