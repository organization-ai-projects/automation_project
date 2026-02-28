use crate::constraints::constraint::Constraint;
use crate::search::evolution_engine::{EvolutionEngine, SearchConfig};
use crate::seed::seed::Seed;

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
fn test_deterministic_search_same_seed_same_hash() {
    let config1 = make_tiny_config(42);
    let mut engine1 = EvolutionEngine::new(config1);
    engine1.run_to_end();
    let manifest1 = engine1.build_candidate_manifest(3);

    let config2 = make_tiny_config(42);
    let mut engine2 = EvolutionEngine::new(config2);
    engine2.run_to_end();
    let manifest2 = engine2.build_candidate_manifest(3);

    assert_eq!(manifest1.manifest_hash, manifest2.manifest_hash);
}

#[test]
fn test_different_seed_different_hash() {
    let config1 = make_tiny_config(42);
    let mut engine1 = EvolutionEngine::new(config1);
    engine1.run_to_end();
    let manifest1 = engine1.build_candidate_manifest(3);

    let config2 = make_tiny_config(999);
    let mut engine2 = EvolutionEngine::new(config2);
    engine2.run_to_end();
    let manifest2 = engine2.build_candidate_manifest(3);

    assert_ne!(manifest1.manifest_hash, manifest2.manifest_hash);
}

#[test]
fn test_golden_manifest_hash() {
    let config = make_tiny_config(42);
    let mut engine = EvolutionEngine::new(config);
    engine.run_to_end();
    let manifest = engine.build_candidate_manifest(3);
    let golden = manifest.manifest_hash.0.clone();

    let config2 = make_tiny_config(42);
    let mut engine2 = EvolutionEngine::new(config2);
    engine2.run_to_end();
    let manifest2 = engine2.build_candidate_manifest(3);

    assert_eq!(manifest2.manifest_hash.0, golden);
}
