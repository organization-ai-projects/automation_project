#![allow(dead_code)]
use crate::config::kernel_config::KernelConfig;
use crate::io::json_codec;
use crate::packs::pack_registry::PackRegistry;
use crate::protocol::message::Message;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::report::run_report::RunReport;
use crate::snapshot::state_snapshot::StateSnapshot;

pub struct RequestDispatcher {
    config: KernelConfig,
    registry: PackRegistry,
    shutdown: bool,
    run_state: Option<RunState>,
}

struct RunState {
    report: RunReport,
    snapshot: StateSnapshot,
}

impl RequestDispatcher {
    pub fn new() -> Self {
        Self {
            config: KernelConfig::default(),
            registry: PackRegistry::new(),
            shutdown: false,
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

        let req = match json_codec::decode::<Request>(&json_codec::encode(&msg.payload).unwrap_or_default()) {
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
            Request::Shutdown => {
                self.shutdown = true;
                Response::ok(id)
            }
            Request::NewRun { pack_kind, seed, ticks, turns, ticks_per_turn } => {
                self.start_run(id, pack_kind, seed, ticks, turns, ticks_per_turn)
            }
            Request::GetReport => {
                if let Some(state) = &self.run_state {
                    Response::report(id, &state.report)
                } else {
                    Response::error(Some(id), "NO_RUN", "No run in progress", "")
                }
            }
            Request::GetSnapshot { at_tick, at_turn } => {
                if let Some(state) = &self.run_state {
                    Response::snapshot(id, &state.snapshot, at_tick, at_turn)
                } else {
                    Response::error(Some(id), "NO_RUN", "No run in progress", "")
                }
            }
            _ => Response::ok(id),
        }
    }

    fn start_run(&mut self, id: u64, pack_kind: String, seed: u64, ticks: u64, _turns: u64, _ticks_per_turn: u64) -> Response {
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

        let pack = match self.registry.get_pack(&pack_kind) {
            Some(p) => p,
            None => {
                return Response::error(Some(id), "UNKNOWN_PACK", &format!("Unknown pack: {}", pack_kind), "");
            }
        };

        pack.initialize(&mut world, seed_val);

        for _tick in 0..ticks {
            clock.advance_tick();
            pack.tick(&mut world, &clock, &mut event_log);
        }

        let snapshot = StateSnapshot::from_world(&world, &clock);
        let snapshot_hash = SnapshotHash::compute(&snapshot);
        let run_hash = RunHash::compute(&event_log, &snapshot_hash);
        let report = RunReport::new(pack_kind, seed, ticks, run_hash, event_log.len());

        self.run_state = Some(RunState { report: report.clone(), snapshot });
        Response::report(id, &report)
    }
}
