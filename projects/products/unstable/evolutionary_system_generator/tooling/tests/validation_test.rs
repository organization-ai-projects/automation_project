use evolutionary_system_generator_tooling::validate::determinism_validator::{DeterminismValidator, ValidatorConfig};

fn make_tiny_config(seed: u64) -> ValidatorConfig {
    ValidatorConfig {
        seed,
        population_size: 6,
        max_generations: 3,
        rule_pool: vec![
            "rule_alpha".to_string(),
            "rule_beta".to_string(),
            "rule_gamma".to_string(),
            "rule_delta".to_string(),
        ],
        constraints: vec![],
    }
}

#[test]
fn test_determinism_validator_passes_same_seed() {
    let result = DeterminismValidator::validate(make_tiny_config(42));
    assert!(result.is_ok());
    assert!(result.unwrap().determinism_ok);
}
