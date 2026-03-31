use crate::replay::replay_engine::ReplayEngine;
use crate::replay::replay_file::ReplayFile;
use crate::rng::rng_draw::RngDraw;
use crate::rng::seed::Seed;
use crate::scenario::scenario_loader::ScenarioLoader;
use crate::turn::turn_engine::TurnEngine;

#[test]
fn replay_produces_identical_hash() {
    let scenario = ScenarioLoader::default_scenario("skirmish").unwrap();
    let seed = Seed(42);
    let (original_report, draws) = TurnEngine::run_battle(&scenario, seed).unwrap();

    let replay_file = ReplayFile {
        seed,
        scenario,
        rng_draws: draws,
    };

    let replayed_report = ReplayEngine::replay(&replay_file).unwrap();
    assert_eq!(original_report.run_hash.0, replayed_report.run_hash.0);
    assert_eq!(original_report.turns_played, replayed_report.turns_played);
    assert_eq!(original_report.winner, replayed_report.winner);
}

#[test]
fn replay_detects_rng_mismatch() {
    let scenario = ScenarioLoader::default_scenario("skirmish").unwrap();
    let seed = Seed(42);
    let (_report, mut draws) = TurnEngine::run_battle(&scenario, seed).unwrap();

    // Inject a fake draw to guarantee mismatch
    draws.push(RngDraw {
        context: "tampered".to_string(),
        value: 999,
    });

    let replay_file = ReplayFile {
        seed,
        scenario,
        rng_draws: draws,
    };

    let result = ReplayEngine::replay(&replay_file);
    assert!(result.is_err());
}
