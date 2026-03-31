use crate::combat::CombatEngine;
use crate::scenarios::ScenarioLoader;

#[test]
fn deterministic_run_produces_same_hash() {
    let scenario = ScenarioLoader::default_scenario("arena_basic");
    let (report1, draws1) = CombatEngine::run(&scenario, 50, 12345).unwrap();
    let (report2, draws2) = CombatEngine::run(&scenario, 50, 12345).unwrap();

    assert_eq!(report1.run_hash.0, report2.run_hash.0);
    assert_eq!(draws1, draws2);
    assert_eq!(report1.enemies_killed, report2.enemies_killed);
    assert_eq!(report1.player_survived, report2.player_survived);
}

#[test]
fn different_seeds_produce_different_hashes() {
    let scenario = ScenarioLoader::default_scenario("arena_basic");
    let (report1, _) = CombatEngine::run(&scenario, 50, 1).unwrap();
    let (report2, _) = CombatEngine::run(&scenario, 50, 2).unwrap();

    assert_ne!(report1.run_hash.0, report2.run_hash.0);
}

#[test]
fn all_rng_draws_are_logged() {
    let scenario = ScenarioLoader::default_scenario("arena_basic");
    let (report, draws) = CombatEngine::run(&scenario, 20, 42).unwrap();

    assert!(!draws.is_empty(), "expected at least one RNG draw");
    assert!(report.event_count > 0, "expected at least one event");
}

#[test]
fn snapshot_hashes_computed_every_10_ticks() {
    use crate::scenarios::Scenario;
    use crate::scenarios::WaveTemplate;

    // Use a scenario with very durable enemies to ensure 20+ ticks
    let scenario = Scenario {
        name: "snapshot_test".to_string(),
        player_hp: 200,
        player_attack: 5,
        player_defense: 10,
        waves: vec![
            WaveTemplate { enemy_count: 5, enemy_hp: 100, enemy_attack: 1, enemy_defense: 3 },
            WaveTemplate { enemy_count: 5, enemy_hp: 100, enemy_attack: 1, enemy_defense: 3 },
        ],
    };
    let (report, _) = CombatEngine::run(&scenario, 30, 42).unwrap();

    assert!(
        report.snapshot_hashes.contains_key("tick_10"),
        "expected snapshot at tick 10"
    );
    assert!(
        report.snapshot_hashes.contains_key("tick_20"),
        "expected snapshot at tick 20"
    );
}
