// projects/products/unstable/hospital_tycoon/backend/src/public_api.rs
use crate::config::sim_config::SimConfig;
use crate::diagnostics::app_error::AppError;
use crate::io::json_codec::JsonCodec;
use crate::protocol::message::Message;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::replay::replay_codec::ReplayCodec;
use crate::replay::replay_engine::ReplayEngine;
use crate::replay::replay_file::ReplayFile;
use crate::report::run_report::RunReport;
use crate::sim::sim_engine::SimEngine;
use crate::snapshot::state_snapshot::StateSnapshot;
use std::path::PathBuf;

pub struct BackendApi {
    state: ServerState,
}

impl BackendApi {
    pub fn from_scenario_path(scenario_path: PathBuf) -> Result<Self, AppError> {
        let config = SimConfig::load(&scenario_path)?;
        Ok(Self {
            state: ServerState::new(config),
        })
    }

    pub fn handle_line(&mut self, line: &str) -> Response {
        let msg: Message = match JsonCodec::decode_message(line) {
            Ok(message) => message,
            Err(error) => return Response::error(None, "PROTOCOL_ERROR", &error.to_string()),
        };
        self.state.handle(msg)
    }
}

struct ServerState {
    config: SimConfig,
    engine: Option<SimEngine>,
    replay_file: Option<ReplayFile>,
}

impl ServerState {
    fn new(config: SimConfig) -> Self {
        Self {
            config,
            engine: None,
            replay_file: None,
        }
    }

    fn handle(&mut self, msg: Message) -> Response {
        match msg.request {
            Request::Ping => Response::ok(msg.id),
            Request::LoadScenario { path } => self.load_scenario(msg.id, path),
            Request::NewRun { seed, ticks } => {
                self.new_run(msg.id, seed.max(0.0) as u64, ticks.max(0.0) as u64)
            }
            Request::Step { n } => self.step(msg.id, n.max(0.0) as u64),
            Request::RunToEnd => self.run_to_end(msg.id),
            Request::GetSnapshot { at_tick } => self.get_snapshot(msg.id, at_tick.max(0.0) as u64),
            Request::GetReport => self.get_report(msg.id),
            Request::SaveReplay { path } => self.save_replay(msg.id, path),
            Request::LoadReplay { path } => self.load_replay(msg.id, path),
            Request::ReplayToEnd => self.replay_to_end(msg.id),
        }
    }

    fn load_scenario(&mut self, id: Option<u64>, path: String) -> Response {
        match SimConfig::load(&std::path::PathBuf::from(&path)) {
            Ok(cfg) => {
                self.config = cfg;
                Response::ok(id)
            }
            Err(e) => Response::error(id, "SCENARIO_ERROR", &e.to_string()),
        }
    }

    fn new_run(&mut self, id: Option<u64>, seed: u64, ticks: u64) -> Response {
        let mut cfg = self.config.clone();
        cfg.seed = seed;
        cfg.ticks = ticks;
        let engine = SimEngine::new(seed, ticks, cfg);
        self.replay_file = Some(ReplayFile::new(seed, ticks, self.config.name.clone()));
        self.engine = Some(engine);
        Response::ok(id)
    }

    fn step(&mut self, id: Option<u64>, n: u64) -> Response {
        let engine = match self.engine.as_mut() {
            Some(e) => e,
            None => return Response::error(id, "NO_RUN", "no active run"),
        };
        for _ in 0..n {
            if engine.clock.is_done() {
                break;
            }
            engine.step_one();
        }
        let snap = StateSnapshot::capture(&engine.state, &engine.event_log);
        Response::snapshot(id, snap)
    }

    fn run_to_end(&mut self, id: Option<u64>) -> Response {
        let engine = match self.engine.as_mut() {
            Some(e) => e,
            None => return Response::error(id, "NO_RUN", "no active run"),
        };
        while !engine.clock.is_done() {
            engine.step_one();
        }
        let snap = StateSnapshot::capture(&engine.state, &engine.event_log);
        Response::snapshot(id, snap)
    }

    fn get_snapshot(&mut self, id: Option<u64>, at_tick: u64) -> Response {
        let engine = match self.engine.as_ref() {
            Some(e) => e,
            None => return Response::error(id, "NO_RUN", "no active run"),
        };
        if at_tick > engine.clock.current_tick().value() {
            return Response::error(id, "SNAPSHOT_OUT_OF_RANGE", "requested tick is not reached");
        }
        let snap = StateSnapshot::capture(&engine.state, &engine.event_log);
        Response::snapshot(id, snap)
    }

    fn get_report(&mut self, id: Option<u64>) -> Response {
        let engine = match self.engine.as_ref() {
            Some(e) => e,
            None => return Response::error(id, "NO_RUN", "no active run"),
        };
        let report = RunReport::generate(
            &engine.state,
            &engine.clock,
            &engine.event_log,
            &self.config.name,
        );
        Response::report(id, report)
    }

    fn save_replay(&mut self, id: Option<u64>, path: String) -> Response {
        let rf = match self.replay_file.as_ref() {
            Some(r) => r,
            None => return Response::error(id, "NO_REPLAY", "no replay data"),
        };
        match ReplayCodec::save(rf, &path) {
            Ok(_) => Response::ok(id),
            Err(e) => Response::error(id, "IO_ERROR", &e.to_string()),
        }
    }

    fn load_replay(&mut self, id: Option<u64>, path: String) -> Response {
        match ReplayCodec::load(&path) {
            Ok(rf) => {
                self.replay_file = Some(rf);
                Response::ok(id)
            }
            Err(e) => Response::error(id, "REPLAY_ERROR", &e.to_string()),
        }
    }

    fn replay_to_end(&mut self, id: Option<u64>) -> Response {
        let rf = match self.replay_file.as_ref() {
            Some(r) => r.clone(),
            None => return Response::error(id, "NO_REPLAY", "no replay loaded"),
        };
        let (state, clock, event_log, report) = ReplayEngine::run(&rf, &self.config);
        self.engine = Some(SimEngine::new(rf.seed, rf.ticks, self.config.clone()));
        if let Some(engine) = self.engine.as_mut() {
            engine.state = state;
            engine.clock = clock;
            engine.event_log = event_log;
        }
        Response::report(id, report)
    }
}
