use crate::adjudication::adjudication_engine::AdjudicationEngine;
use crate::ai::ai_engine::AiEngine;
use crate::ai::ai_profile::AiProfile;
use crate::map::map_loader::load_map_from_str;
use crate::map::territory_id::TerritoryId;
use crate::model::faction::Faction;
use crate::model::faction_id::FactionId;
use crate::model::game_state::GameState;
use crate::model::unit::Unit;
use crate::model::unit_id::UnitId;
use crate::replay::event_log::EventLog;
use crate::replay::replay_engine::replay;
use crate::replay::replay_event::ReplayEvent;
use crate::replay::replay_file::ReplayFile;
use crate::report::match_report::MatchReport;
use crate::report::run_hash::compute_run_hash_from_json;
use crate::report::turn_report::TurnReport;
use crate::time::turn::Turn;

const TINY_MAP_JSON: &str = include_str!("fixtures/maps/tiny_triangle_map.json");
const NUM_TURNS: u32 = 5;
const SEED: u64 = 42;

fn run_tiny_triangle() -> (MatchReport, ReplayFile) {
    let (map, starting_units) = load_map_from_str(TINY_MAP_JSON).unwrap();

    let factions = vec![
        Faction {
            id: FactionId(0),
            name: "Faction0".into(),
        },
        Faction {
            id: FactionId(1),
            name: "Faction1".into(),
        },
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
        map_graph: map,
    };

    let mut engine = AdjudicationEngine::new(initial_state);
    let ai = AiEngine::new(SEED, AiProfile::default());
    let mut event_log = EventLog::new();
    let mut turn_reports = Vec::<TurnReport>::new();
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

    let replayed_report = replay(&replay_file).expect("replay should succeed");

    assert_eq!(original_report.run_hash, replayed_report.run_hash);

    let original_json = crate::report::run_hash::canonical_json_string(
        &common_json::to_json(&original_report).unwrap(),
    );
    let replayed_json = crate::report::run_hash::canonical_json_string(
        &common_json::to_json(&replayed_report).unwrap(),
    );
    assert_eq!(original_json, replayed_json);
}
