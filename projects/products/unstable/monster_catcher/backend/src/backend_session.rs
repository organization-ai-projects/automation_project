use crate::capture::capture_engine::CaptureEngine;
use crate::combat::action::BattleAction;
use crate::combat::battle_id::BattleId;
use crate::combat::battle_report::BattleReport;
use crate::combat::battle_state::BattleState;
use crate::combat::combat_engine::CombatEngine;
use crate::data::data_store::DataStore;
use crate::data::move_id::MoveId;
use crate::diagnostics::error::BackendError;
use crate::encounter::encounter_engine::EncounterEngine;
use crate::encounter::encounter_table::EncounterTable;
use crate::events::event_log::EventLog;
use crate::events::game_event::GameEvent;
use crate::io::canonical_json::to_canonical_string;
use crate::model::inventory::Inventory;
use crate::model::item_id::ItemId;
use crate::model::monster::Monster;
use crate::model::monster_id::MonsterId;
use crate::model::party::Party;
use crate::progression::progression_engine::ProgressionEngine;
use crate::protocol::request::Request;
use crate::protocol::request_payload::RequestPayload;
use crate::protocol::response::Response;
use crate::protocol::response_payload::ResponsePayload;
use crate::replay::replay_engine::ReplayEngine;
use crate::report::run_hash::RunHash;
use crate::report::run_report::RunReport;
use crate::rng::seed::Seed;
use crate::scenario::scenario::Scenario;
use crate::scenario::scenario_loader::ScenarioLoader;
use crate::snapshot::snapshot_hash::SnapshotHash;
use crate::snapshot::state_snapshot::StateSnapshot;
use rand::SeedableRng;
use rand::rngs::SmallRng;

#[derive(Debug)]
pub struct BackendSession {
    pub scenario_path: Option<String>,
    pub scenario: Option<Scenario>,
    pub data: DataStore,
    pub encounter_table: EncounterTable,
    pub party: Party,
    pub inventory: Inventory,
    pub event_log: EventLog,
    pub battle_reports: Vec<BattleReport>,
    pub current_battle: Option<BattleState>,
    pub current_encounter: Option<Monster>,
    pub seed: Seed,
    pub step_count: u64,
    pub next_monster_index: u64,
    pub next_battle_index: u64,
    pub rng: Option<SmallRng>,
    pub replay_data: Option<String>,
    pub shutdown: bool,
}

impl BackendSession {
    pub fn new(scenario_path: Option<String>) -> Self {
        Self {
            scenario_path,
            scenario: None,
            data: DataStore::new(),
            encounter_table: EncounterTable::default(),
            party: Party::default(),
            inventory: Inventory::default(),
            event_log: EventLog::default(),
            battle_reports: Vec::new(),
            current_battle: None,
            current_encounter: None,
            seed: Seed::default(),
            step_count: 0,
            next_monster_index: 0,
            next_battle_index: 0,
            rng: None,
            replay_data: None,
            shutdown: false,
        }
    }

    pub fn should_shutdown(&self) -> bool {
        self.shutdown
    }

    pub fn handle(&mut self, request: Request) -> Response {
        let id = request.id;
        let payload = match self.handle_payload(request.payload) {
            Ok(p) => p,
            Err(e) => ResponsePayload::Error {
                message: e.to_string(),
            },
        };
        Response { id, payload }
    }

    fn handle_payload(
        &mut self,
        payload: RequestPayload,
    ) -> Result<ResponsePayload, BackendError> {
        match payload {
            RequestPayload::LoadScenario { scenario: source } => {
                let scenario = if let Some(ref path) = self.scenario_path {
                    ScenarioLoader::load_file(path)?
                } else {
                    ScenarioLoader::load(&source)?
                };
                self.apply_scenario(&scenario);
                self.scenario = Some(scenario);
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::NewRun { seed } => {
                self.seed = Seed::new(seed);
                self.rng = Some(SmallRng::seed_from_u64(seed));
                self.event_log = EventLog::default();
                self.battle_reports = Vec::new();
                self.current_battle = None;
                self.current_encounter = None;
                self.step_count = 0;
                self.next_monster_index = 0;
                self.next_battle_index = 0;

                if let Some(ref scenario) = self.scenario.clone() {
                    self.setup_starter(scenario);
                }

                self.event_log.push(GameEvent::RunStarted { seed });
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::EncounterStep => {
                self.step_count += 1;
                let mut draws = Vec::new();
                let mut rng = self.take_rng()?;
                let monster = EncounterEngine::generate_encounter(
                    &mut rng,
                    &self.encounter_table,
                    &self.data,
                    self.step_count,
                    &mut self.next_monster_index,
                    &mut draws,
                );
                self.put_rng(rng);
                let monster = monster?;
                self.event_log.push_rng_draws(&draws);
                self.event_log.push(GameEvent::EncounterGenerated {
                    species: monster.species_id.0.clone(),
                    level: monster.level,
                });
                let encounter_json =
                    to_canonical_string(&monster).map_err(BackendError::Codec)?;
                self.current_encounter = Some(monster);
                Ok(ResponsePayload::EncounterState { encounter_json })
            }
            RequestPayload::StartEncounter => self.handle_payload(RequestPayload::EncounterStep),
            RequestPayload::CaptureAttempt => {
                let target = self
                    .current_encounter
                    .as_ref()
                    .ok_or_else(|| BackendError::Capture("no active encounter".to_string()))?;
                let species = self
                    .data
                    .get_species(&target.species_id)
                    .ok_or_else(|| BackendError::Data("species not found".to_string()))?;
                let capture_rate = species.capture_rate;

                let pokeball = ItemId("pokeball".to_string());
                if !self.inventory.use_item(&pokeball) {
                    return Err(BackendError::Capture(
                        "no pokeballs available".to_string(),
                    ));
                }

                let mut draws = Vec::new();
                let target_clone = target.clone();
                let mut rng = self.take_rng()?;
                let roll = CaptureEngine::attempt_capture(
                    &mut rng,
                    &target_clone,
                    capture_rate,
                    self.step_count,
                    &mut draws,
                );
                self.put_rng(rng);
                self.event_log.push_rng_draws(&draws);
                self.event_log.push(GameEvent::CaptureAttempted {
                    success: roll.success,
                    roll: roll.roll,
                    threshold: roll.threshold,
                });

                if roll.success {
                    if let Some(monster) = self.current_encounter.take() {
                        self.party.add(monster);
                    }
                }

                let result_json = to_canonical_string(&roll).map_err(BackendError::Codec)?;
                Ok(ResponsePayload::EncounterState {
                    encounter_json: result_json,
                })
            }
            RequestPayload::StartBattle => {
                let enemy = self
                    .current_encounter
                    .take()
                    .ok_or_else(|| BackendError::Combat("no encounter to battle".to_string()))?;
                let player = self
                    .party
                    .first_alive()
                    .ok_or_else(|| {
                        BackendError::Combat("no alive monster in party".to_string())
                    })?
                    .clone();

                self.next_battle_index += 1;
                let battle_id = BattleId(format!("battle_{}", self.next_battle_index));
                self.event_log.push(GameEvent::BattleStarted {
                    battle_id: battle_id.0.clone(),
                });

                let battle = BattleState::new(battle_id, player, enemy);
                let battle_json = to_canonical_string(&battle).map_err(BackendError::Codec)?;
                self.current_battle = Some(battle);
                Ok(ResponsePayload::BattleState { battle_json })
            }
            RequestPayload::BattleAction {
                action: action_str,
            } => {
                let action = parse_battle_action(&action_str)?;
                self.execute_battle_turn(action)
            }
            RequestPayload::BattleStep => {
                let battle = self
                    .current_battle
                    .as_ref()
                    .ok_or_else(|| BackendError::Combat("no active battle".to_string()))?;
                let player_move = battle
                    .player_monster
                    .moves
                    .first()
                    .cloned()
                    .unwrap_or_else(|| MoveId("tackle".to_string()));
                let action = BattleAction::UseMove {
                    move_id: player_move,
                };
                self.execute_battle_turn(action)
            }
            RequestPayload::EndBattle => {
                let battle = self
                    .current_battle
                    .take()
                    .ok_or_else(|| BackendError::Combat("no active battle".to_string()))?;
                let player_won = battle.player_won.unwrap_or(false);
                self.event_log
                    .push(GameEvent::BattleEnded { player_won });

                let mut xp_gained = 0u64;
                if player_won {
                    let enemy_species =
                        self.data.get_species(&battle.enemy_monster.species_id);
                    if let Some(species) = enemy_species {
                        let base_yield = species.base_xp_yield;
                        let enemy_level = battle.enemy_monster.level;
                        if let Some(player_mon) = self.party.first_alive_mut() {
                            let (gain, new_moves) = ProgressionEngine::award_xp(
                                player_mon,
                                base_yield,
                                enemy_level,
                                &self.data,
                            )?;
                            xp_gained = gain.xp_gained;
                            self.event_log.push(GameEvent::XpAwarded {
                                amount: xp_gained,
                                new_level: player_mon.level,
                            });
                            for m in &new_moves {
                                self.event_log.push(GameEvent::MoveUnlocked {
                                    move_id: m.0.clone(),
                                });
                            }
                        }
                    }
                }

                let report = CombatEngine::build_report(&battle, xp_gained);
                self.battle_reports.push(report);

                Ok(ResponsePayload::Ok)
            }
            RequestPayload::GetSnapshot => {
                let snapshot = StateSnapshot {
                    seed: self.seed.value,
                    step_count: self.step_count,
                    party: self.party.clone(),
                    inventory: self.inventory.clone(),
                    events: self.event_log.clone(),
                };
                let hash = SnapshotHash::compute(&snapshot)?;
                let state_json =
                    to_canonical_string(&snapshot).map_err(BackendError::Codec)?;
                Ok(ResponsePayload::Snapshot { hash, state_json })
            }
            RequestPayload::GetReport => {
                let report = RunReport::build(
                    self.seed.value,
                    self.step_count,
                    &self.party,
                    &self.event_log,
                    &self.battle_reports,
                );
                let run_hash = RunHash::compute(&report)?;
                let report_json =
                    to_canonical_string(&report).map_err(BackendError::Codec)?;
                Ok(ResponsePayload::Report {
                    run_hash,
                    report_json,
                })
            }
            RequestPayload::SaveReplay => {
                let replay = ReplayEngine::build_replay(
                    self.seed.value,
                    self.step_count,
                    &self.party,
                    &self.inventory,
                    &self.event_log,
                )?;
                self.replay_data = Some(replay.clone());
                Ok(ResponsePayload::ReplayData { replay })
            }
            RequestPayload::LoadReplay { replay } => {
                self.replay_data = Some(replay);
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::ReplayToEnd => {
                let raw = self
                    .replay_data
                    .as_ref()
                    .ok_or_else(|| {
                        BackendError::Replay("no replay data loaded".to_string())
                    })?
                    .clone();
                let (party, inventory, event_log, step_count, seed) =
                    ReplayEngine::replay_to_end(&raw)?;
                self.party = party;
                self.inventory = inventory;
                self.event_log = event_log;
                self.step_count = step_count;
                self.seed = Seed::new(seed);
                Ok(ResponsePayload::Ok)
            }
            RequestPayload::Shutdown => {
                self.shutdown = true;
                Ok(ResponsePayload::Ok)
            }
        }
    }

    fn execute_battle_turn(
        &mut self,
        action: BattleAction,
    ) -> Result<ResponsePayload, BackendError> {
        let mut battle = self
            .current_battle
            .take()
            .ok_or_else(|| BackendError::Combat("no active battle".to_string()))?;

        let mut draws = Vec::new();
        self.step_count += 1;
        let mut rng = self.take_rng()?;
        let result = CombatEngine::execute_turn(
            &mut rng,
            &mut battle,
            action,
            &self.data,
            self.step_count,
            &mut draws,
        );
        self.put_rng(rng);
        result?;

        self.event_log.push_rng_draws(&draws);
        self.event_log.push(GameEvent::BattleTurn {
            turn_number: battle.turns.len() as u32,
        });

        let battle_json = to_canonical_string(&battle).map_err(BackendError::Codec)?;
        self.current_battle = Some(battle);
        Ok(ResponsePayload::BattleState { battle_json })
    }

    fn apply_scenario(&mut self, scenario: &Scenario) {
        self.data = DataStore::new();
        for species in &scenario.species {
            self.data.add_species(species.clone());
        }
        for move_data in &scenario.moves {
            self.data.add_move(move_data.clone());
        }
        for entry in &scenario.type_effectiveness {
            self.data
                .type_chart
                .set(&entry.attacker, &entry.defender, entry.factor);
        }
        self.encounter_table = scenario.encounter_table.clone();
        self.event_log.push(GameEvent::ScenarioLoaded {
            name: scenario.name.clone(),
        });
    }

    fn setup_starter(&mut self, scenario: &Scenario) {
        let species = self.data.get_species(&scenario.starter_species_id);
        if let Some(s) = species {
            let moves = s
                .learnset
                .iter()
                .filter(|e| e.level <= scenario.starter_level)
                .map(|e| e.move_id.clone())
                .collect();
            self.next_monster_index += 1;
            let starter = Monster::new(
                MonsterId(format!("mon_{}", self.next_monster_index)),
                scenario.starter_species_id.clone(),
                scenario.starter_level,
                s.base_hp,
                s.base_attack,
                s.base_defense,
                s.base_speed,
                moves,
            );
            self.party = Party::default();
            self.party.add(starter);
        }
        self.inventory = Inventory::default();
        self.inventory
            .add(&ItemId("pokeball".to_string()), scenario.initial_pokeballs);
    }

    fn take_rng(&mut self) -> Result<SmallRng, BackendError> {
        self.rng.take().ok_or_else(|| {
            BackendError::Engine("no RNG initialized; call NewRun first".to_string())
        })
    }

    fn put_rng(&mut self, rng: SmallRng) {
        self.rng = Some(rng);
    }
}

fn parse_battle_action(s: &str) -> Result<BattleAction, BackendError> {
    if s == "flee" {
        return Ok(BattleAction::Flee);
    }
    Ok(BattleAction::UseMove {
        move_id: MoveId(s.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::request::Request;
    use crate::protocol::request_payload::RequestPayload;
    use crate::protocol::response_payload::ResponsePayload;

    fn request(payload: RequestPayload) -> Request {
        Request {
            id: Some("test".to_string()),
            payload,
        }
    }

    #[test]
    fn load_scenario_and_new_run() {
        let mut session = BackendSession::new(None);
        let r = session.handle(request(RequestPayload::LoadScenario {
            scenario: "default".to_string(),
        }));
        assert!(matches!(r.payload, ResponsePayload::Ok));

        let r = session.handle(request(RequestPayload::NewRun { seed: 42 }));
        assert!(matches!(r.payload, ResponsePayload::Ok));
        assert!(!session.party.monsters.is_empty());
    }

    #[test]
    fn deterministic_encounter() {
        let mut s1 = BackendSession::new(None);
        let mut s2 = BackendSession::new(None);

        for s in [&mut s1, &mut s2] {
            s.handle(request(RequestPayload::LoadScenario {
                scenario: "default".to_string(),
            }));
            s.handle(request(RequestPayload::NewRun { seed: 42 }));
        }

        let r1 = s1.handle(request(RequestPayload::EncounterStep));
        let r2 = s2.handle(request(RequestPayload::EncounterStep));
        let e1 = extract_encounter(&r1.payload);
        let e2 = extract_encounter(&r2.payload);
        assert_eq!(e1, e2);
    }

    #[test]
    fn deterministic_battle() {
        let mut s1 = BackendSession::new(None);
        let mut s2 = BackendSession::new(None);

        for s in [&mut s1, &mut s2] {
            s.handle(request(RequestPayload::LoadScenario {
                scenario: "default".to_string(),
            }));
            s.handle(request(RequestPayload::NewRun { seed: 42 }));
            s.handle(request(RequestPayload::EncounterStep));
            s.handle(request(RequestPayload::StartBattle));
            s.handle(request(RequestPayload::BattleStep));
            s.handle(request(RequestPayload::BattleStep));
            s.handle(request(RequestPayload::BattleStep));
        }

        let r1 = s1.handle(request(RequestPayload::GetSnapshot));
        let r2 = s2.handle(request(RequestPayload::GetSnapshot));
        let (h1, _) = extract_snapshot(&r1.payload);
        let (h2, _) = extract_snapshot(&r2.payload);
        assert_eq!(h1, h2);
    }

    #[test]
    fn capture_attempt() {
        let mut session = BackendSession::new(None);
        session.handle(request(RequestPayload::LoadScenario {
            scenario: "default".to_string(),
        }));
        session.handle(request(RequestPayload::NewRun { seed: 42 }));
        session.handle(request(RequestPayload::EncounterStep));

        let r = session.handle(request(RequestPayload::CaptureAttempt));
        match &r.payload {
            ResponsePayload::EncounterState { encounter_json } => {
                assert!(!encounter_json.is_empty());
            }
            ResponsePayload::Error { message } => {
                panic!("capture failed: {message}");
            }
            _ => panic!("unexpected response"),
        }
    }

    #[test]
    fn run_report_deterministic() {
        let mut s1 = BackendSession::new(None);
        let mut s2 = BackendSession::new(None);

        for s in [&mut s1, &mut s2] {
            s.handle(request(RequestPayload::LoadScenario {
                scenario: "default".to_string(),
            }));
            s.handle(request(RequestPayload::NewRun { seed: 99 }));
            s.handle(request(RequestPayload::EncounterStep));
            s.handle(request(RequestPayload::StartBattle));
            for _ in 0..20 {
                let r = s.handle(request(RequestPayload::BattleStep));
                if let ResponsePayload::BattleState { battle_json } = &r.payload {
                    if battle_json.contains("\"finished\":true")
                        || battle_json.contains("\"finished\": true")
                    {
                        break;
                    }
                }
            }
            s.handle(request(RequestPayload::EndBattle));
        }

        let r1 = s1.handle(request(RequestPayload::GetReport));
        let r2 = s2.handle(request(RequestPayload::GetReport));
        let (h1, _) = extract_report(&r1.payload);
        let (h2, _) = extract_report(&r2.payload);
        assert_eq!(h1, h2);
    }

    #[test]
    fn replay_roundtrip() {
        let mut session = BackendSession::new(None);
        session.handle(request(RequestPayload::LoadScenario {
            scenario: "default".to_string(),
        }));
        session.handle(request(RequestPayload::NewRun { seed: 77 }));
        session.handle(request(RequestPayload::EncounterStep));
        session.handle(request(RequestPayload::CaptureAttempt));

        let snap1 = session.handle(request(RequestPayload::GetSnapshot));
        let (h1, _) = extract_snapshot(&snap1.payload);

        let r = session.handle(request(RequestPayload::SaveReplay));
        let replay = extract_replay(&r.payload);

        session.handle(request(RequestPayload::LoadReplay { replay }));
        session.handle(request(RequestPayload::ReplayToEnd));

        let snap2 = session.handle(request(RequestPayload::GetSnapshot));
        let (h2, _) = extract_snapshot(&snap2.payload);
        assert_eq!(h1, h2);
    }

    #[test]
    fn rng_draws_logged() {
        let mut session = BackendSession::new(None);
        session.handle(request(RequestPayload::LoadScenario {
            scenario: "default".to_string(),
        }));
        session.handle(request(RequestPayload::NewRun { seed: 42 }));
        session.handle(request(RequestPayload::EncounterStep));

        let has_rng_draw = session
            .event_log
            .events
            .iter()
            .any(|e| matches!(e, GameEvent::RngDraw { .. }));
        assert!(has_rng_draw, "RNG draws must be logged as events");
    }

    #[test]
    fn progression_xp_and_level() {
        let mut session = BackendSession::new(None);
        session.handle(request(RequestPayload::LoadScenario {
            scenario: "default".to_string(),
        }));
        session.handle(request(RequestPayload::NewRun { seed: 42 }));

        for _ in 0..5 {
            session.handle(request(RequestPayload::EncounterStep));
            session.handle(request(RequestPayload::StartBattle));
            for _ in 0..30 {
                let r = session.handle(request(RequestPayload::BattleStep));
                if let ResponsePayload::BattleState { battle_json } = &r.payload {
                    if battle_json.contains("\"finished\":true")
                        || battle_json.contains("\"finished\": true")
                    {
                        break;
                    }
                }
                if session
                    .current_battle
                    .as_ref()
                    .map_or(true, |b| b.finished)
                {
                    break;
                }
            }
            session.handle(request(RequestPayload::EndBattle));

            if session.party.all_fainted() {
                break;
            }
        }

        let xp_events = session
            .event_log
            .events
            .iter()
            .filter(|e| matches!(e, GameEvent::XpAwarded { .. }))
            .count();
        assert!(xp_events > 0, "should have XP award events");
    }

    fn extract_encounter(payload: &ResponsePayload) -> String {
        match payload {
            ResponsePayload::EncounterState { encounter_json } => encounter_json.clone(),
            other => panic!("expected EncounterState, got {other:?}"),
        }
    }

    fn extract_snapshot(payload: &ResponsePayload) -> (String, String) {
        match payload {
            ResponsePayload::Snapshot { hash, state_json } => {
                (hash.clone(), state_json.clone())
            }
            other => panic!("expected Snapshot, got {other:?}"),
        }
    }

    fn extract_report(payload: &ResponsePayload) -> (String, String) {
        match payload {
            ResponsePayload::Report {
                run_hash,
                report_json,
            } => (run_hash.clone(), report_json.clone()),
            other => panic!("expected Report, got {other:?}"),
        }
    }

    fn extract_replay(payload: &ResponsePayload) -> String {
        match payload {
            ResponsePayload::ReplayData { replay } => replay.clone(),
            other => panic!("expected ReplayData, got {other:?}"),
        }
    }
}
