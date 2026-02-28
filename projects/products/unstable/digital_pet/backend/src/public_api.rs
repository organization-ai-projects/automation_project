// projects/products/unstable/digital_pet/backend/src/public_api.rs
use crate::battle::battle_engine::BattleEngine;
use crate::care::care_engine::CareEngine;
use crate::config::sim_config::SimConfig;
use crate::diagnostics::error::AppError;
use crate::events::event_log::EventLog;
use crate::evolution::evolution_engine::EvolutionEngine;
use crate::evolution::evolution_tree::EvolutionTree;
use crate::io::json_codec::JsonCodec;
use crate::model::pet::Pet;
use crate::model::pet_state::PetState;
use crate::needs::needs_state::NeedsState;
use crate::protocol::message::Message;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::replay::replay_engine::ReplayEngine;
use crate::replay::replay_file::ReplayFile;
use crate::report::run_report::RunReport;
use crate::scenario::scenario::Scenario;
use crate::scenario::scenario_loader::ScenarioLoader;
use crate::snapshot::state_snapshot::StateSnapshot;
use crate::time::tick_clock::TickClock;
use crate::training::training_engine::TrainingEngine;
use std::io::{BufRead, Write};
use std::path::PathBuf;

pub struct BackendApi;

impl BackendApi {
    pub fn serve(scenario_path: PathBuf) -> Result<(), AppError> {
        let scenario = ScenarioLoader::load(&scenario_path)?;
        let mut state = ServerState::new(scenario);
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();
        let mut out = std::io::BufWriter::new(stdout.lock());
        for line in stdin.lock().lines() {
            let line = line.map_err(|e| AppError::Io(e.to_string()))?;
            if line.trim().is_empty() {
                continue;
            }
            let msg: Message = match JsonCodec::decode_message(&line) {
                Ok(m) => m,
                Err(e) => {
                    let resp = Response::error(None, &e.to_string());
                    let encoded = JsonCodec::encode_response(&resp)?;
                    writeln!(out, "{}", encoded).map_err(|e| AppError::Io(e.to_string()))?;
                    out.flush().map_err(|e| AppError::Io(e.to_string()))?;
                    continue;
                }
            };
            let resp = state.handle(msg);
            let encoded = JsonCodec::encode_response(&resp)?;
            writeln!(out, "{}", encoded).map_err(|e| AppError::Io(e.to_string()))?;
            out.flush().map_err(|e| AppError::Io(e.to_string()))?;
        }
        Ok(())
    }
}

struct ServerState {
    scenario: Scenario,
    clock: Option<TickClock>,
    pet: Option<Pet>,
    needs: Option<NeedsState>,
    event_log: EventLog,
    care_engine: CareEngine,
    evolution_engine: EvolutionEngine,
    training_engine: TrainingEngine,
    battle_engine: Option<BattleEngine>,
    replay_file: Option<ReplayFile>,
    replay_mode: bool,
}

impl ServerState {
    fn new(scenario: Scenario) -> Self {
        let tree = EvolutionTree::from_config(&scenario.config);
        Self {
            scenario,
            clock: None,
            pet: None,
            needs: None,
            event_log: EventLog::new(),
            care_engine: CareEngine::new(),
            evolution_engine: EvolutionEngine::new(tree),
            training_engine: TrainingEngine::new(),
            battle_engine: None,
            replay_file: None,
            replay_mode: false,
        }
    }

    fn handle(&mut self, msg: Message) -> Response {
        match msg.request {
            Request::NewRun { seed, ticks } => self.new_run(msg.id, seed, ticks),
            Request::Step { n } => self.step(msg.id, n),
            Request::CareAction { kind } => self.care_action(msg.id, kind),
            Request::Training { kind } => self.training(msg.id, kind),
            Request::StartBattle => self.start_battle(msg.id),
            Request::BattleStep => self.battle_step(msg.id),
            Request::GetSnapshot => self.get_snapshot(msg.id),
            Request::GetReport => self.get_report(msg.id),
            Request::SaveReplay { path } => self.save_replay(msg.id, path),
            Request::LoadReplay { path } => self.load_replay(msg.id, path),
            Request::ReplayToEnd => self.replay_to_end(msg.id),
            Request::LoadScenario { path } => self.load_scenario_req(msg.id, path),
        }
    }

    fn new_run(&mut self, id: Option<u64>, seed: u64, ticks: u64) -> Response {
        let config = SimConfig::default();
        self.clock = Some(TickClock::new(seed, ticks));
        let species = self.scenario.starting_species.clone();
        self.pet = Some(Pet::new(seed, species));
        self.needs = Some(NeedsState::default());
        self.event_log = EventLog::new();
        self.care_engine = CareEngine::new();
        self.replay_file = Some(ReplayFile::new(seed, ticks, self.scenario.clone()));
        self.replay_mode = false;
        let _ = config;
        Response::ok(id)
    }

    fn step(&mut self, id: Option<u64>, n: u64) -> Response {
        let clock = match self.clock.as_mut() {
            Some(c) => c,
            None => return Response::error(id, "no active run"),
        };
        let pet = match self.pet.as_mut() {
            Some(p) => p,
            None => return Response::error(id, "no active pet"),
        };
        let needs = match self.needs.as_mut() {
            Some(n) => n,
            None => return Response::error(id, "no active needs"),
        };
        for _ in 0..n {
            if clock.is_done() {
                break;
            }
            clock.tick();
            needs.decay(clock.current_tick());
            self.care_engine.evaluate(needs, clock.current_tick());
            self.evolution_engine.evaluate(
                pet,
                needs,
                &self.care_engine,
                clock.current_tick(),
                &mut self.event_log,
            );
        }
        let state = PetState::from_pet_and_needs(pet, needs, clock.current_tick());
        Response::pet_state(id, state)
    }

    fn care_action(&mut self, id: Option<u64>, kind: String) -> Response {
        let needs = match self.needs.as_mut() {
            Some(n) => n,
            None => return Response::error(id, "no active needs"),
        };
        let clock = match self.clock.as_ref() {
            Some(c) => c,
            None => return Response::error(id, "no active run"),
        };
        self.care_engine
            .apply_action(kind, needs, clock.current_tick());
        Response::ok(id)
    }

    fn training(&mut self, id: Option<u64>, kind: String) -> Response {
        let pet = match self.pet.as_mut() {
            Some(p) => p,
            None => return Response::error(id, "no active pet"),
        };
        let clock = match self.clock.as_ref() {
            Some(c) => c,
            None => return Response::error(id, "no active run"),
        };
        let result = self.training_engine.train(pet, &kind, clock.current_tick());
        Response::ok_with_data(id, format!("training: {:?}", result))
    }

    fn start_battle(&mut self, id: Option<u64>) -> Response {
        let pet = match self.pet.as_ref() {
            Some(p) => p,
            None => return Response::error(id, "no active pet"),
        };
        let clock = match self.clock.as_ref() {
            Some(c) => c,
            None => return Response::error(id, "no active run"),
        };
        self.battle_engine = Some(BattleEngine::new(pet.clone(), clock.current_tick()));
        Response::ok(id)
    }

    fn battle_step(&mut self, id: Option<u64>) -> Response {
        let engine = match self.battle_engine.as_mut() {
            Some(e) => e,
            None => return Response::error(id, "no active battle"),
        };
        let state = engine.step();
        Response::battle_state(id, state)
    }

    fn get_snapshot(&mut self, id: Option<u64>) -> Response {
        let pet = match self.pet.as_ref() {
            Some(p) => p,
            None => return Response::error(id, "no active pet"),
        };
        let needs = match self.needs.as_ref() {
            Some(n) => n,
            None => return Response::error(id, "no active needs"),
        };
        let clock = match self.clock.as_ref() {
            Some(c) => c,
            None => return Response::error(id, "no active run"),
        };
        let snapshot = StateSnapshot::capture(pet, needs, clock.current_tick(), &self.event_log);
        Response::snapshot(id, snapshot)
    }

    fn get_report(&mut self, id: Option<u64>) -> Response {
        let pet = match self.pet.as_ref() {
            Some(p) => p,
            None => return Response::error(id, "no active pet"),
        };
        let needs = match self.needs.as_ref() {
            Some(n) => n,
            None => return Response::error(id, "no active needs"),
        };
        let clock = match self.clock.as_ref() {
            Some(c) => c,
            None => return Response::error(id, "no active run"),
        };
        let report = RunReport::generate(pet, needs, clock, &self.event_log, &self.care_engine);
        Response::report(id, report)
    }

    fn save_replay(&mut self, id: Option<u64>, path: String) -> Response {
        use crate::replay::replay_codec::ReplayCodec;
        let rf = match self.replay_file.as_ref() {
            Some(r) => r,
            None => return Response::error(id, "no replay data"),
        };
        match ReplayCodec::save(rf, &path) {
            Ok(_) => Response::ok(id),
            Err(e) => Response::error(id, &e.to_string()),
        }
    }

    fn load_replay(&mut self, id: Option<u64>, path: String) -> Response {
        use crate::replay::replay_codec::ReplayCodec;
        match ReplayCodec::load(&path) {
            Ok(rf) => {
                self.replay_file = Some(rf);
                self.replay_mode = true;
                Response::ok(id)
            }
            Err(e) => Response::error(id, &e.to_string()),
        }
    }

    fn replay_to_end(&mut self, id: Option<u64>) -> Response {
        let rf = match self.replay_file.as_ref() {
            Some(r) => r.clone(),
            None => return Response::error(id, "no replay loaded"),
        };
        let (pet, needs, clock, event_log, care_engine) = ReplayEngine::run(&rf);
        let report = RunReport::generate(&pet, &needs, &clock, &event_log, &care_engine);
        self.pet = Some(pet);
        self.needs = Some(needs);
        self.clock = Some(clock);
        self.event_log = event_log;
        self.care_engine = care_engine;
        Response::report(id, report)
    }

    fn load_scenario_req(&mut self, id: Option<u64>, path: String) -> Response {
        match ScenarioLoader::load(&std::path::PathBuf::from(path)) {
            Ok(s) => {
                let tree = EvolutionTree::from_config(&s.config);
                self.scenario = s;
                self.evolution_engine = EvolutionEngine::new(tree);
                Response::ok(id)
            }
            Err(e) => Response::error(id, &e.to_string()),
        }
    }
}
