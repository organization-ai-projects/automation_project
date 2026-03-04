// projects/products/unstable/evolutionary_system_generator/backend/src/search/search_config.rs
use crate::constraints::constraint::Constraint;
use crate::seed::Seed;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub seed: Seed,
    pub population_size: usize,
    pub max_generations: u32,
    pub rule_pool: Vec<String>,
    pub constraints: Vec<Constraint>,
}
