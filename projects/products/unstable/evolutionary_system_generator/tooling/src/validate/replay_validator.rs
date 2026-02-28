use evolutionary_system_generator_backend::public_api::{Constraint, EvolutionEngine, ReplayEngine, SearchConfig, Seed, EventLog};
use crate::validate::determinism_validator::ValidatorConfig;

#[derive(Debug)]
pub struct ReplayValidatorResult {
    pub replay_ok: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ReplayValidatorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Replay failed: {0}")]
    Replay(String),
}

pub struct ReplayValidator;

impl ReplayValidator {
    pub fn validate(config: ValidatorConfig, replay_path: &str) -> Result<ReplayValidatorResult, ReplayValidatorError> {
        let sc = SearchConfig {
            seed: Seed(config.seed),
            population_size: config.population_size,
            max_generations: config.max_generations,
            rule_pool: config.rule_pool.clone(),
            constraints: config.constraints.clone(),
        };
        let mut engine = EvolutionEngine::new(sc);
        engine.run_to_end();
        let original_manifest = engine.build_candidate_manifest(5);
        engine.get_event_log().save_to_file(replay_path)?;

        let log = EventLog::load_from_file(replay_path)?;
        let result = ReplayEngine::replay_from_log(&log, config.rule_pool.clone(), config.constraints.clone(), 5)
            .map_err(|e| ReplayValidatorError::Replay(e.to_string()))?;

        let ok = result.manifest.manifest_hash == original_manifest.manifest_hash;
        Ok(ReplayValidatorResult { replay_ok: ok })
    }
}
