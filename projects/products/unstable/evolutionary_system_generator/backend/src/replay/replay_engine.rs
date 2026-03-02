use crate::constraints::constraint::Constraint;
use crate::output::candidate_manifest::CandidateManifest;
use crate::replay::event_log::EventLog;
use crate::replay::search_event::SearchEventKind;
use crate::search::evolution_engine::{EvolutionEngine, SearchConfig};
use crate::seed::seed::Seed;

#[derive(Debug)]
pub struct ReplayResult {
    pub matches: bool,
    pub original_event_count: usize,
    pub replayed_event_count: usize,
    pub manifest: CandidateManifest,
}

#[derive(Debug, thiserror::Error)]
pub enum ReplayError {
    #[error("SearchStarted event not found in log")]
    NoSearchStartedEvent,
    #[error("Event count mismatch: original {0}, replayed {1}")]
    EventCountMismatch(usize, usize),
}

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay_from_log(
        log: &EventLog,
        rule_pool: Vec<String>,
        constraints: Vec<Constraint>,
        top_n: usize,
    ) -> Result<ReplayResult, ReplayError> {
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

        let replayed_log = engine.get_event_log();
        let matches = log.events.len() == replayed_log.events.len();

        let manifest = engine.build_candidate_manifest(top_n);

        Ok(ReplayResult {
            matches,
            original_event_count: log.events.len(),
            replayed_event_count: replayed_log.events.len(),
            manifest,
        })
    }
}
