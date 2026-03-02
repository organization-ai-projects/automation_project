#![allow(dead_code)]
use crate::config::sim_config::SimConfig;
use crate::diagnostics::error::SimError;
use crate::events::event_log::EventLog;
use crate::io::json_codec;
use crate::protocol::message::Message;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::replay::replay_codec::ReplayCodec;
use crate::replay::replay_file::ReplayFile;
use crate::report::run_report::RunReport;
use crate::scenario::scenario::Scenario;
use crate::scenario::scenario_loader::ScenarioLoader;
use crate::sim::sim_engine::SimEngine;
use crate::sim::sim_state::SimState;
use crate::snapshot::state_snapshot::StateSnapshot;

pub struct RequestDispatcher {
    preload_scenario_path: Option<String>,
    scenario: Option<Scenario>,
    run_state: Option<RunState>,
    shutdown: bool,
    config: SimConfig,
}

struct RunState {
    engine: SimEngine,
    state: SimState,
    event_log: EventLog,
    report: Option<RunReport>,
    ticks_target: u64,
    seed: u64,
    scenario_id: String,
    scenario_hash: String,
}

impl RequestDispatcher {
    pub fn new(preload_scenario_path: Option<String>) -> Self {
        let mut d = Self {
            preload_scenario_path: preload_scenario_path.clone(),
            scenario: None,
            run_state: None,
            shutdown: false,
            config: SimConfig::default(),
        };
        if let Some(path) = preload_scenario_path {
            match ScenarioLoader::load_from_file(&path) {
                Ok(s) => {
                    tracing::info!("Pre-loaded scenario: {}", s.id);
                    d.scenario = Some(s);
                }
                Err(e) => {
                    tracing::error!("Failed to pre-load scenario {}: {}", path, e);
                    std::process::exit(3);
                }
            }
        }
        d
    }

    pub fn dispatch(&mut self, line: &str) -> String {
        let msg: Message = match json_codec::decode(line) {
            Ok(m) => m,
            Err(e) => {
                let resp = Response::error(None, "PARSE_ERROR", &e.to_string(), "");
                return json_codec::encode(&resp).unwrap_or_else(|_| "{}".to_string());
            }
        };

        let req: Request = match serde_json::from_value(msg.payload) {
            Ok(r) => r,
            Err(e) => {
                let resp = Response::error(Some(msg.id), "INVALID_REQUEST", &e.to_string(), "");
                return json_codec::encode(&resp).unwrap_or_else(|_| "{}".to_string());
            }
        };

        let response = self.handle_request(msg.id, req);
        json_codec::encode(&response).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn should_shutdown(&self) -> bool {
        self.shutdown
    }

    fn handle_request(&mut self, id: u64, req: Request) -> Response {
        match req {
            Request::Ping => Response::ok(id),
            Request::Shutdown => {
                self.shutdown = true;
                Response::ok(id)
            }
            Request::LoadScenario { path } => self.handle_load_scenario(id, &path),
            Request::NewRun { seed, ticks } => self.handle_new_run(id, seed, ticks),
            Request::Step { n } => self.handle_step(id, n),
            Request::RunToEnd => self.handle_run_to_end(id),
            Request::GetSnapshot { at_tick } => self.handle_get_snapshot(id, at_tick),
            Request::GetReport => self.handle_get_report(id),
            Request::SaveReplay { path } => self.handle_save_replay(id, &path),
            Request::LoadReplay { path } => self.handle_load_replay(id, &path),
            Request::ReplayToEnd => self.handle_replay_to_end(id),
        }
    }

    fn handle_load_scenario(&mut self, id: u64, path: &str) -> Response {
        match ScenarioLoader::load_from_file(path) {
            Ok(s) => {
                tracing::info!("Loaded scenario: {}", s.id);
                self.scenario = Some(s);
                self.run_state = None;
                Response::ok(id)
            }
            Err(SimError::Io(e)) => Response::error(Some(id), "IO_ERROR", &e, ""),
            Err(SimError::InvalidScenario(e)) => {
                Response::error(Some(id), "INVALID_SCENARIO", &e, "")
            }
            Err(e) => Response::error(Some(id), "INTERNAL_ERROR", &e.to_string(), ""),
        }
    }

    fn handle_new_run(&mut self, id: u64, seed: u64, ticks: u64) -> Response {
        let scenario = match &self.scenario {
            Some(s) => s.clone(),
            None => {
                return Response::error(Some(id), "NO_SCENARIO", "No scenario loaded", "");
            }
        };

        let scenario_id = scenario.id.clone();
        let scenario_hash = scenario.hash();
        let (engine, state) = SimEngine::new(&scenario, seed, &self.config);
        self.run_state = Some(RunState {
            engine,
            state,
            event_log: EventLog::new(),
            report: None,
            ticks_target: ticks,
            seed,
            scenario_id,
            scenario_hash,
        });

        tracing::info!("New run: seed={} ticks={}", seed, ticks);
        Response::ok(id)
    }

    fn handle_step(&mut self, id: u64, n: u64) -> Response {
        let rs = match &mut self.run_state {
            Some(rs) => rs,
            None => {
                return Response::error(Some(id), "NO_RUN", "No run in progress", "");
            }
        };

        let current_tick = rs.state.clock.tick.value();
        let stop_at = (current_tick + n).min(rs.ticks_target);
        while rs.state.clock.tick.value() < stop_at {
            rs.engine.tick(&mut rs.state, &mut rs.event_log);
        }

        let snap = StateSnapshot::from_state(&rs.state);
        Response::snapshot(id, &snap)
    }

    fn handle_run_to_end(&mut self, id: u64) -> Response {
        let rs = match &mut self.run_state {
            Some(rs) => rs,
            None => {
                return Response::error(Some(id), "NO_RUN", "No run in progress", "");
            }
        };

        while rs.state.clock.tick.value() < rs.ticks_target {
            rs.engine.tick(&mut rs.state, &mut rs.event_log);
        }

        let report = RunReport::build_with_scenario(
            &rs.state,
            &rs.event_log,
            rs.ticks_target,
            &rs.scenario_id,
            &rs.scenario_hash,
            rs.seed,
        );
        rs.report = Some(report.clone());
        Response::report(id, &report)
    }

    fn handle_get_snapshot(&mut self, id: u64, _at_tick: u64) -> Response {
        let rs = match &self.run_state {
            Some(rs) => rs,
            None => {
                return Response::error(Some(id), "NO_RUN", "No run in progress", "");
            }
        };
        let snap = StateSnapshot::from_state(&rs.state);
        Response::snapshot(id, &snap)
    }

    fn handle_get_report(&mut self, id: u64) -> Response {
        let rs = match &self.run_state {
            Some(rs) => rs,
            None => {
                return Response::error(Some(id), "NO_RUN", "No run in progress", "");
            }
        };
        let report = match &rs.report {
            Some(r) => r.clone(),
            None => RunReport::build_with_scenario(
                &rs.state,
                &rs.event_log,
                rs.ticks_target,
                &rs.scenario_id,
                &rs.scenario_hash,
                rs.seed,
            ),
        };
        Response::report(id, &report)
    }

    fn handle_save_replay(&mut self, id: u64, path: &str) -> Response {
        let rs = match &self.run_state {
            Some(rs) => rs,
            None => {
                return Response::error(Some(id), "NO_RUN", "No run in progress", "");
            }
        };
        let scenario = match &self.scenario {
            Some(s) => s.clone(),
            None => {
                return Response::error(Some(id), "NO_SCENARIO", "No scenario loaded", "");
            }
        };
        let replay = ReplayFile::from_run(&scenario, &rs.state, &rs.event_log);
        match ReplayCodec::save(&replay, path) {
            Ok(_) => Response::ok(id),
            Err(e) => Response::error(Some(id), "IO_ERROR", &e.to_string(), ""),
        }
    }

    fn handle_load_replay(&mut self, id: u64, path: &str) -> Response {
        match ReplayCodec::load(path) {
            Ok(replay) => {
                tracing::info!("Loaded replay from {}", path);
                let scenario = Scenario::from_replay(&replay);
                let scenario_id = scenario.id.clone();
                let scenario_hash = scenario.hash();
                let seed = replay.seed;
                let ticks = replay.ticks;
                self.scenario = Some(scenario.clone());
                let (engine, state) = SimEngine::new(&scenario, seed, &self.config);
                self.run_state = Some(RunState {
                    engine,
                    state,
                    event_log: EventLog::new(),
                    report: None,
                    ticks_target: ticks,
                    seed,
                    scenario_id,
                    scenario_hash,
                });
                Response::ok(id)
            }
            Err(e) => Response::error(Some(id), "IO_ERROR", &e.to_string(), ""),
        }
    }

    fn handle_replay_to_end(&mut self, id: u64) -> Response {
        match &mut self.run_state {
            None => Response::error(Some(id), "NO_RUN", "No run in progress", ""),
            Some(rs) => {
                while rs.state.clock.tick.value() < rs.ticks_target {
                    rs.engine.tick(&mut rs.state, &mut rs.event_log);
                }
                let report = RunReport::build_with_scenario(
                    &rs.state,
                    &rs.event_log,
                    rs.ticks_target,
                    &rs.scenario_id,
                    &rs.scenario_hash,
                    rs.seed,
                );
                rs.report = Some(report.clone());
                Response::report(id, &report)
            }
        }
    }
}
