// projects/products/unstable/evolutionary_system_generator/backend/src/tooling/validator_config.rs
#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    pub seed: u64,
    pub population_size: usize,
    pub max_generations: u32,
    pub rule_pool: Vec<String>,
}
