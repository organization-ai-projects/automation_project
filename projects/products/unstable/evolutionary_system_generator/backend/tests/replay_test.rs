use evolutionary_system_generator_backend::public_api::{
    Constraint, EvolutionEngine, ReplayEngine, SearchConfig, Seed,
};

fn make_tiny_config(seed: u64) -> SearchConfig {
    SearchConfig {
        seed: Seed(seed),
        population_size: 6,
        max_generations: 3,
        rule_pool: vec![
            "rule_alpha".to_string(),
            "rule_beta".to_string(),
            "rule_gamma".to_string(),
            "rule_delta".to_string(),
        ],
        constraints: vec![
            Constraint::MinActiveRules(2),
            Constraint::MaxTotalWeight(30),
        ],
    }
}

#[test]
fn test_replay_produces_identical_trajectory() {
    let config = make_tiny_config(42);
    let mut engine = EvolutionEngine::new(config.clone());
    engine.run_to_end();
    let original_log = engine.get_event_log().clone();
    let original_manifest = engine.build_candidate_manifest(3);

    let result = ReplayEngine::replay_from_log(
        &original_log,
        config.rule_pool.clone(),
        config.constraints.clone(),
        3,
    )
    .unwrap();

    assert!(result.matches);
    assert_eq!(result.manifest.manifest_hash, original_manifest.manifest_hash);
}
