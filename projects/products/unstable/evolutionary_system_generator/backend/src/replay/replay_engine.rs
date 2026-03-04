// projects/products/unstable/evolutionary_system_generator/backend/src/replay/replay_engine.rs
use crate::constraints::constraint::Constraint;
use crate::replay::event_log::EventLog;
use crate::replay::replay_error::ReplayError;
use crate::replay::replay_result::ReplayResult;
use crate::replay::search_event_kind::SearchEventKind;
use crate::search::evolution_engine::EvolutionEngine;
use crate::search::search_config::SearchConfig;
use crate::seed::Seed;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay_search(
        log: &EventLog,
        rule_pool: Vec<String>,
        constraints: Vec<Constraint>,
    ) -> Result<EvolutionEngine, ReplayError> {
        let started = log.events.iter().find_map(|e| {
            if let SearchEventKind::SearchStarted {
                seed,
                population_size,
                max_generations,
            } = &e.kind
            {
                Some((*seed, *population_size, *max_generations))
            } else {
                None
            }
        });

        let (seed, population_size, max_generations) =
            started.ok_or(ReplayError::NoSearchStartedEvent)?;

        let config = SearchConfig {
            seed: Seed(seed),
            population_size,
            max_generations,
            rule_pool,
            constraints,
        };

        let mut engine = EvolutionEngine::new(config);
        engine.run_to_end();
        Ok(engine)
    }

    pub fn replay_from_log(
        log: &EventLog,
        rule_pool: Vec<String>,
        constraints: Vec<Constraint>,
        top_n: usize,
    ) -> Result<ReplayResult, ReplayError> {
        let engine = Self::replay_search(log, rule_pool, constraints)?;

        let replayed_log = engine.get_event_log();
        if log.events.len() != replayed_log.events.len() {
            return Err(ReplayError::EventCountMismatch(
                log.events.len(),
                replayed_log.events.len(),
            ));
        }

        let manifest = engine.build_candidate_manifest(top_n);

        Ok(ReplayResult {
            matches: true,
            original_event_count: log.events.len(),
            replayed_event_count: replayed_log.events.len(),
            manifest,
        })
    }
}
