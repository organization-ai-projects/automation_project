use diplo_sim::adjudication::adjudication_engine::AdjudicationEngine;
use diplo_sim::ai::ai_engine::AiEngine;
use diplo_sim::ai::ai_profile::AiProfile;
use diplo_sim::map::map_loader::load_map_from_str;
use diplo_sim::map::territory_id::TerritoryId;
use diplo_sim::model::faction::Faction;
use diplo_sim::model::faction_id::FactionId;
use diplo_sim::model::game_state::GameState;
use diplo_sim::model::unit::Unit;
use diplo_sim::model::unit_id::UnitId;
use diplo_sim::replay::event_log::EventLog;
use diplo_sim::replay::replay_engine::replay;
use diplo_sim::replay::replay_event::ReplayEvent;
use diplo_sim::replay::replay_file::ReplayFile;
use diplo_sim::report::match_report::MatchReport;
use diplo_sim::report::run_hash::compute_run_hash_from_json;
use diplo_sim::report::turn_report::TurnReport;
use diplo_sim::time::turn::Turn;

const TINY_MAP_JSON: &str = include_str!("fixtures/maps/tiny_triangle_map.json");
const NUM_TURNS: u32 = 5;
const SEED: u64 = 42;

fn run_tiny_triangle() -> (MatchReport, ReplayFile) {
    let (map, starting_units) = load_map_from_str(TINY_MAP_JSON).unwrap();

    let factions: Vec<Faction> = vec![
        Faction { id: FactionId(0), name: "Faction0".into() },
        Faction { id: FactionId(1), name: "Faction1".into() },
    ];
    let units: Vec<Unit> = starting_units
        .iter()
        .enumerate()
        .map(|(i, su)| Unit {
            id: UnitId(i as u32),
            faction_id: FactionId(su.faction_id),
            territory_id: TerritoryId(su.territory_id),
        })
        .collect();

    let initial_state = GameState {
        units,
        factions,
        current_turn: Turn::new(0),
        map_graph: map.clone(),
    };

    let mut engine = AdjudicationEngine::new(initial_state);
    let ai = AiEngine::new(SEED, AiProfile::default());
    let mut event_log = EventLog::new();
    let mut turn_reports: Vec<TurnReport> = Vec::new();
    let mut next_order_id: u32 = 0;

    for _ in 0..NUM_TURNS {
        let turn = engine.state.current_turn;
        let order_sets = ai.generate_all_orders(&engine.state, &mut next_order_id);
        let adjudication = engine.adjudicate(&order_sets);

        event_log.push(ReplayEvent {
            turn,
            order_sets: order_sets.clone(),
        });

        turn_reports.push(TurnReport {
            turn,
            order_sets,
            adjudication,
        });
    }

    let map_hash = compute_run_hash_from_json(TINY_MAP_JSON);
    let match_report = MatchReport::build("tiny_triangle".to_string(), SEED, turn_reports);
    let replay_file = ReplayFile {
        map_hash,
        map_name: "tiny_triangle".to_string(),
        map_json: TINY_MAP_JSON.to_string(),
        seed: SEED,
        num_factions: 2,
        event_log,
    };

    (match_report, replay_file)
}

#[test]
fn test_replay_produces_identical_match_report() {
    let (original_report, replay_file) = run_tiny_triangle();

    let replayed_report = replay(&replay_file).expect("Replay should succeed");

    // Compare run hashes
    assert_eq!(
        original_report.run_hash, replayed_report.run_hash,
        "RunHash must be identical after replay"
    );

    // Compare full serialized JSON bytes using canonical JSON
    let original_json = diplo_sim::report::run_hash::canonical_json_string(
        &common_json::to_json(&original_report).unwrap()
    );
    let replayed_json = diplo_sim::report::run_hash::canonical_json_string(
        &common_json::to_json(&replayed_report).unwrap()
    );
    assert_eq!(
        original_json, replayed_json,
        "Serialized MatchReport must be byte-identical after replay"
    );
}

#[test]
fn test_replay_run_hash_matches() {
    let (original_report, replay_file) = run_tiny_triangle();
    let replayed_report = replay(&replay_file).expect("Replay should succeed");

    assert_eq!(original_report.run_hash, replayed_report.run_hash);
    assert!(!original_report.run_hash.is_empty());
}

#[test]
fn test_replay_has_correct_turn_count() {
    let (_, replay_file) = run_tiny_triangle();
    let replayed = replay(&replay_file).unwrap();
    assert_eq!(replayed.turns.len(), NUM_TURNS as usize);
}
