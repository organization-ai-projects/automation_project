use evolutionary_system_generator_backend::public_api::{Constraint, EvolutionEngine, SearchConfig, Seed};

#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    pub seed: u64,
    pub population_size: usize,
    pub max_generations: u32,
    pub rule_pool: Vec<String>,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug)]
pub struct DeterminismResult {
    pub determinism_ok: bool,
    pub manifest_hash: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ValidatorError {
    #[error("Determinism check failed: hashes differ")]
    HashMismatch,
}

pub struct DeterminismValidator;

impl DeterminismValidator {
    pub fn validate(config: ValidatorConfig) -> Result<DeterminismResult, ValidatorError> {
        let make_engine = |c: &ValidatorConfig| {
            let sc = SearchConfig {
                seed: Seed(c.seed),
                population_size: c.population_size,
                max_generations: c.max_generations,
                rule_pool: c.rule_pool.clone(),
                constraints: c.constraints.clone(),
            };
            let mut engine = EvolutionEngine::new(sc);
            engine.run_to_end();
            engine
        };

        let mut engine1 = make_engine(&config);
        let manifest1 = engine1.build_candidate_manifest(5);

        let mut engine2 = make_engine(&config);
        let manifest2 = engine2.build_candidate_manifest(5);

        let ok = manifest1.manifest_hash == manifest2.manifest_hash;
        Ok(DeterminismResult {
            determinism_ok: ok,
            manifest_hash: Some(manifest1.manifest_hash.0),
        })
    }
}
