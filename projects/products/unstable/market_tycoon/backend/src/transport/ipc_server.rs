use std::io::BufRead;

use crate::config::sim_config::SimConfig;
use crate::diagnostics::error::Error;
use crate::events::event_log::EventLog;
use crate::io::json_codec::JsonCodec;
use crate::replay::replay_codec::ReplayCodec;
use crate::replay::replay_file::ReplayFile;
use crate::report::sim_report::SimReport;
use crate::scenario::scenario::Scenario;
use crate::snapshot::state_snapshot::StateSnapshot;
use crate::transport::request::{Message, Request};
use crate::transport::response::Response;

pub struct IpcServer {
    scenario: Scenario,
    engine: Option<crate::SimEngine>,
    event_log: Option<EventLog>,
    config: Option<SimConfig>,
}

impl IpcServer {
    pub fn run(scenario: Scenario) -> Result<(), Error> {
        let mut server = Self {
            scenario,
            engine: None,
            event_log: None,
            config: None,
        };

        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let line = line.map_err(|e| Error::Io(e.to_string()))?;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let msg: Message = match common_json::from_str(trimmed) {
                Ok(m) => m,
                Err(e) => {
                    let resp = Response::error(0, "PROTOCOL_ERROR", &e.to_string());
                    let json = common_json::to_string(&resp).unwrap_or_default();
                    println!("{json}");
                    continue;
                }
            };

            let resp = server.handle(msg);
            let json =
                common_json::to_string(&resp).map_err(|e| Error::Serialization(e.to_string()))?;
            println!("{json}");
        }

        Ok(())
    }

    fn handle(&mut self, msg: Message) -> Response {
        match msg.request {
            Request::Ping => Response::ok(msg.id),
            Request::NewRun { seed, ticks } => self.new_run(msg.id, seed, ticks),
            Request::Step { n } => self.step(msg.id, n),
            Request::RunToEnd => self.run_to_end(msg.id),
            Request::GetSnapshot { at_tick } => self.get_snapshot(msg.id, at_tick),
            Request::GetReport => self.get_report(msg.id),
            Request::SaveReplay { ref path } => self.save_replay(msg.id, path),
            Request::LoadReplay { ref path } => self.load_replay(msg.id, path),
            Request::ReplayToEnd => self.replay_to_end(msg.id),
        }
    }

    fn new_run(&mut self, id: u64, seed: u64, ticks: u64) -> Response {
        let config = SimConfig::from_scenario(&self.scenario, seed, ticks);
        let engine = crate::create_engine(&config);
        self.engine = Some(engine);
        self.event_log = Some(EventLog::new());
        self.config = Some(config);
        Response::ok(id)
    }

    fn step(&mut self, id: u64, n: u64) -> Response {
        let (engine, config, event_log) =
            match (&mut self.engine, &self.config, &mut self.event_log) {
                (Some(e), Some(c), Some(el)) => (e, c, el),
                _ => return Response::error(id, "NO_RUN", "no active run"),
            };

        for _ in 0..n {
            if engine.clock.is_done() {
                break;
            }
            run_single_tick(engine, config, event_log);
        }

        let snap = StateSnapshot::capture(engine, event_log);
        Response::snapshot(id, snap.hash, snap.tick)
    }

    fn run_to_end(&mut self, id: u64) -> Response {
        let (engine, config, event_log) =
            match (&mut self.engine, &self.config, &mut self.event_log) {
                (Some(e), Some(c), Some(el)) => (e, c, el),
                _ => return Response::error(id, "NO_RUN", "no active run"),
            };

        while !engine.clock.is_done() {
            run_single_tick(engine, config, event_log);
        }

        let snap = StateSnapshot::capture(engine, event_log);
        Response::snapshot(id, snap.hash, snap.tick)
    }

    fn get_snapshot(&self, id: u64, _at_tick: u64) -> Response {
        let (engine, event_log) = match (&self.engine, &self.event_log) {
            (Some(e), Some(el)) => (e, el),
            _ => return Response::error(id, "NO_RUN", "no active run"),
        };
        let snap = StateSnapshot::capture(engine, event_log);
        Response::snapshot(id, snap.hash, snap.tick)
    }

    fn get_report(&self, id: u64) -> Response {
        let (engine, config, event_log) = match (&self.engine, &self.config, &self.event_log) {
            (Some(e), Some(c), Some(el)) => (e, c, el),
            _ => return Response::error(id, "NO_RUN", "no active run"),
        };
        let report = SimReport::generate(engine, event_log, config);
        let json = JsonCodec::encode(&report).unwrap_or_default();
        Response::report(id, report.run_hash, json)
    }

    fn save_replay(&self, id: u64, path: &str) -> Response {
        let (config, event_log) = match (&self.config, &self.event_log) {
            (Some(c), Some(el)) => (c, el),
            _ => return Response::error(id, "NO_RUN", "no active run"),
        };
        let rf = ReplayFile::new(config.seed, config.ticks, event_log.events().to_vec());
        match ReplayCodec::encode(&rf) {
            Ok(json) => match std::fs::write(path, json) {
                Ok(()) => Response::ok(id),
                Err(e) => Response::error(id, "IO_ERROR", &e.to_string()),
            },
            Err(e) => Response::error(id, "ENCODE_ERROR", &e.to_string()),
        }
    }

    fn load_replay(&mut self, id: u64, path: &str) -> Response {
        match std::fs::read_to_string(path) {
            Ok(data) => match ReplayCodec::decode(&data) {
                Ok(rf) => {
                    let config = SimConfig::from_scenario(&self.scenario, rf.seed, rf.ticks);
                    let engine = crate::create_engine(&config);
                    self.engine = Some(engine);
                    self.event_log = Some(EventLog::new());
                    self.config = Some(config);
                    Response::ok(id)
                }
                Err(e) => Response::error(id, "DECODE_ERROR", &e.to_string()),
            },
            Err(e) => Response::error(id, "IO_ERROR", &e.to_string()),
        }
    }

    fn replay_to_end(&mut self, id: u64) -> Response {
        self.run_to_end(id)
    }
}

fn run_single_tick(engine: &mut crate::SimEngine, config: &SimConfig, event_log: &mut EventLog) {
    let tick = engine.clock.current();

    for contract in &config.contracts {
        crate::supply::supply_engine::SupplyEngine::process_delivery(
            contract,
            &tick,
            &mut engine.inventories,
            &mut engine.ledger,
            event_log,
        );
    }

    for (sid, inv) in &engine.inventories {
        let store = &engine.stores[sid];
        let company = &engine.companies[&store.owner()];
        crate::pricing::pricing_engine::PricingEngine::update_prices(
            &tick,
            company,
            inv,
            &config.pricing_policy,
            event_log,
        );
    }

    let rng_val = crate::next_rng(&mut engine.rng_state);
    crate::demand::demand_engine::DemandEngine::process_demand(
        &tick,
        &config.demand_model,
        &config.segments,
        rng_val,
        &mut engine.inventories,
        &mut engine.ledger,
        event_log,
    );

    crate::logistics::logistics_engine::LogisticsEngine::process_shipments(
        &tick,
        &config.contracts,
        &mut engine.inventories,
        event_log,
    );

    engine.clock.advance();
}
