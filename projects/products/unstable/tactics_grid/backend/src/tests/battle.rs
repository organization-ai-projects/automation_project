use crate::rng::seed::Seed;
use crate::scenario::scenario_loader::ScenarioLoader;
use crate::turn::turn_engine::TurnEngine;

#[test]
fn skirmish_battle_completes() {
    let scenario = ScenarioLoader::default_scenario("skirmish").unwrap();
    let (report, _draws) = TurnEngine::run_battle(&scenario, Seed(42)).unwrap();
    assert!(report.turns_played > 0);
    assert!(!report.run_hash.0.is_empty());
}

#[test]
fn skirmish_battle_is_deterministic() {
    let scenario = ScenarioLoader::default_scenario("skirmish").unwrap();
    let (report1, draws1) = TurnEngine::run_battle(&scenario, Seed(42)).unwrap();
    let (report2, draws2) = TurnEngine::run_battle(&scenario, Seed(42)).unwrap();
    assert_eq!(report1.run_hash.0, report2.run_hash.0);
    assert_eq!(draws1, draws2);
    assert_eq!(report1.turns_played, report2.turns_played);
    assert_eq!(report1.winner, report2.winner);
}

#[test]
fn different_seeds_produce_different_results() {
    let scenario = ScenarioLoader::default_scenario("skirmish").unwrap();
    let (report1, _) = TurnEngine::run_battle(&scenario, Seed(1)).unwrap();
    let (report2, _) = TurnEngine::run_battle(&scenario, Seed(999)).unwrap();
    assert_ne!(report1.run_hash.0, report2.run_hash.0);
}

#[test]
fn battle_produces_snapshot_hashes() {
    let scenario = ScenarioLoader::default_scenario("skirmish").unwrap();
    let (report, _) = TurnEngine::run_battle(&scenario, Seed(42)).unwrap();
    assert!(report.snapshot_hashes.contains_key("turn_0"));
    assert!(report.snapshot_hashes.len() > 1);
}

#[test]
fn battle_has_actions() {
    let scenario = ScenarioLoader::default_scenario("skirmish").unwrap();
    let (report, _) = TurnEngine::run_battle(&scenario, Seed(42)).unwrap();
    assert!(!report.actions.is_empty());
}

#[test]
fn unknown_scenario_returns_error() {
    let result = ScenarioLoader::default_scenario("nonexistent");
    assert!(result.is_err());
}
