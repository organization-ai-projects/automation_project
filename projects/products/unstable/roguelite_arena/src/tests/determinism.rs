use crate::combat::CombatEngine;
use crate::scenarios::ScenarioLoader;

#[test]
fn full_run_deterministic_across_scenarios() {
    for scenario_name in &["arena_basic", "arena_hard"] {
        let scenario = ScenarioLoader::default_scenario(scenario_name);
        let (r1, d1) = CombatEngine::run(&scenario, 100, 999).unwrap();
        let (r2, d2) = CombatEngine::run(&scenario, 100, 999).unwrap();
        assert_eq!(r1.run_hash.0, r2.run_hash.0, "scenario: {scenario_name}");
        assert_eq!(d1, d2, "draws mismatch for: {scenario_name}");
    }
}

#[test]
fn run_report_has_valid_hash() {
    let scenario = ScenarioLoader::default_scenario("arena_basic");
    let (report, _) = CombatEngine::run(&scenario, 50, 42).unwrap();
    assert!(!report.run_hash.0.is_empty(), "run hash should not be empty");
    assert_eq!(report.run_hash.0.len(), 64, "sha256 hex should be 64 chars");
}
