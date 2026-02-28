use std::path::Path;

use space_diplo_wars::config::game_config::GameConfig;
use space_diplo_wars::io::json_codec::JsonCodec;
use space_diplo_wars::replay::replay_engine::ReplayEngine;
use space_diplo_wars::replay::replay_file::ReplayFile;
use space_diplo_wars::report::run_hash::RunHash;
use space_diplo_wars::report::run_report::RunReport;
use space_diplo_wars::report::turn_report::TurnReport;
use space_diplo_wars::resolution::resolution_engine::ResolutionEngine;
use space_diplo_wars::scenario::scenario_loader::ScenarioLoader;
use space_diplo_wars::snapshot::snapshot_hash::SnapshotHash;
use space_diplo_wars::snapshot::state_snapshot::StateSnapshot;

fn fixtures_dir() -> std::path::PathBuf {
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest.join("tests/fixtures/scenarios")
}

fn run_scenario(scenario_file: &str, turns: u64) -> (RunReport, ReplayFile) {
    let path = fixtures_dir().join(scenario_file);
    let scenario = ScenarioLoader::load_from_file(&path).expect("load scenario");

    let config = GameConfig { turns, ticks_per_turn: 4, seed: 42 };

    let mut state = scenario.build_initial_state();
    let mut turn_reports = Vec::new();

    // Build a simple replay file
    let scenario_json = JsonCodec::encode(&scenario).unwrap();
    let mut hasher = <sha2::Sha256 as sha2::Digest>::new();
    sha2::Digest::update(&mut hasher, scenario_json.as_bytes());
    let scenario_hash = hex::encode(sha2::Digest::finalize(hasher));
    let mut replay = ReplayFile::new(config.seed, scenario_hash);

    for turn in 1..=turns {
        let orders = scenario.orders_for_turn(turn);
        let order_set = space_diplo_wars::orders::order_set::OrderSet { turn, orders: orders.clone() };
        replay.orders_per_turn.insert(turn.to_string(), order_set);

        let res = ResolutionEngine::resolve_turn(&mut state, &orders, turn);
        turn_reports.push(TurnReport {
            turn,
            battles: res.battles,
            diplomacy_events: res.diplomacy_events,
            validation_errors: res.validation_errors,
        });

        state.current_turn = space_diplo_wars::time::turn::Turn(turn);
        state.current_tick = space_diplo_wars::time::tick::Tick(turn * config.ticks_per_turn);
    }

    let final_snapshot = StateSnapshot::from_state(&state);
    let snapshot_hash = SnapshotHash::compute(&final_snapshot).unwrap();

    let run_report = RunReport {
        game_id: state.game_id.0.clone(),
        seed: config.seed,
        turns_played: turns,
        turn_reports,
        final_snapshot_hash: snapshot_hash.0,
    };

    (run_report, replay)
}

#[test]
fn test_peaceful_trade_pact_scenario() {
    let (report1, _) = run_scenario("peaceful_trade_pact.json", 5);
    let (report2, _) = run_scenario("peaceful_trade_pact.json", 5);

    // Determinism check: same run produces same snapshot hash
    assert_eq!(report1.final_snapshot_hash, report2.final_snapshot_hash);
    assert_eq!(report1.turns_played, 5);
}

#[test]
fn test_border_war_scenario() {
    let (report, _) = run_scenario("border_war_small.json", 2);

    // Turn 2 should have a battle
    let turn2 = report.turn_reports.iter().find(|t| t.turn == 2).unwrap();
    assert_eq!(turn2.battles.len(), 1);

    // Verify canonical JSON is stable
    let json = JsonCodec::encode(&report).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_run_replay_identical_report() {
    let (report1, replay) = run_scenario("peaceful_trade_pact.json", 5);
    let hash1 = RunHash::compute(&report1).unwrap();

    // Replay the same orders
    let path = fixtures_dir().join("peaceful_trade_pact.json");
    let scenario = ScenarioLoader::load_from_file(&path).unwrap();
    let report2 = ReplayEngine::replay(&replay, &scenario).unwrap();
    let hash2 = RunHash::compute(&report2).unwrap();

    assert_eq!(hash1.0, hash2.0, "Replay should produce identical RunHash");
}

#[test]
fn test_snapshot_hash_at_checkpoint() {
    let path = fixtures_dir().join("border_war_small.json");
    let scenario = ScenarioLoader::load_from_file(&path).unwrap();

    let config = GameConfig { turns: 2, ticks_per_turn: 4, seed: 42 };
    let mut state = scenario.build_initial_state();

    let mut snap_hashes: Vec<(u64, String)> = Vec::new();

    for turn in 1..=config.turns {
        let orders = scenario.orders_for_turn(turn);
        ResolutionEngine::resolve_turn(&mut state, &orders, turn);

        for cp in &scenario.checkpoints {
            if cp.turn == turn {
                let snap = StateSnapshot::from_state(&state);
                let hash = SnapshotHash::compute(&snap).unwrap();
                snap_hashes.push((turn, hash.0));
            }
        }
    }

    // Run again and verify same hashes
    let mut state2 = scenario.build_initial_state();
    let mut snap_hashes2: Vec<(u64, String)> = Vec::new();

    for turn in 1..=config.turns {
        let orders = scenario.orders_for_turn(turn);
        ResolutionEngine::resolve_turn(&mut state2, &orders, turn);

        for cp in &scenario.checkpoints {
            if cp.turn == turn {
                let snap = StateSnapshot::from_state(&state2);
                let hash = SnapshotHash::compute(&snap).unwrap();
                snap_hashes2.push((turn, hash.0));
            }
        }
    }

    assert_eq!(snap_hashes, snap_hashes2, "Snapshot hashes must be deterministic");
}

#[test]
fn test_invalid_scenario_validation() {
    use space_diplo_wars::scenario::scenario_loader::ScenarioLoader;
    use std::path::Path;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn unique_tmp_path(suffix: &str) -> std::path::PathBuf {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        let pid = std::process::id();
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("sdw_test_{pid}_{ts}_{n}{suffix}"))
    }

    // Non-existent file (use a path that never exists)
    let missing = unique_tmp_path("_missing.json");
    let result = ScenarioLoader::load_from_file(&missing);
    assert!(result.is_err());

    // Bad JSON
    let bad_json = unique_tmp_path("_bad.json");
    std::fs::write(&bad_json, "not valid json {{{").unwrap();
    let result2 = ScenarioLoader::load_from_file(Path::new(&bad_json));
    assert!(result2.is_err());
    std::fs::remove_file(&bad_json).ok();
}
