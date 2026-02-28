use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::{apply_candidates, apply_error, apply_report};
use evolutionary_system_generator_backend::public_api::{Seed, SearchConfig, EvolutionEngine};

pub struct Controller {
    engine: Option<EvolutionEngine>,
}

impl Controller {
    pub fn new() -> Self {
        Self { engine: None }
    }

    pub fn handle(&mut self, state: &mut AppState, action: Action) -> Vec<String> {
        match action {
            Action::StartSearch { seed, population_size, max_generations, rule_pool, constraints } => {
                let config = SearchConfig {
                    seed: Seed(seed),
                    population_size,
                    max_generations,
                    rule_pool,
                    constraints,
                };
                self.engine = Some(EvolutionEngine::new(config));
                vec!["Search started.".to_string()]
            }
            Action::StepGen => {
                if let Some(ref mut eng) = self.engine {
                    let done = eng.step_generation();
                    let pop = eng.get_population();
                    let best = pop.individuals.iter()
                        .map(|i| i.fitness.0)
                        .fold(f64::NEG_INFINITY, f64::max);
                    let generation = pop.generation;
                    apply_report(state, generation, best, done);
                    vec![format!("Generation {}, best_fitness={:.4}, done={}", generation, best, done)]
                } else {
                    apply_error(state, "No search active".to_string());
                    vec!["Error: No search active".to_string()]
                }
            }
            Action::RunToEnd => {
                if let Some(ref mut eng) = self.engine {
                    eng.run_to_end();
                    let pop = eng.get_population();
                    let best = pop.individuals.iter()
                        .map(|i| i.fitness.0)
                        .fold(f64::NEG_INFINITY, f64::max);
                    let generation = pop.generation;
                    apply_report(state, generation, best, true);
                    vec![format!("Done. Generation {}, best_fitness={:.4}", generation, best)]
                } else {
                    apply_error(state, "No search active".to_string());
                    vec!["Error: No search active".to_string()]
                }
            }
            Action::ShowCandidates { top_n } => {
                if let Some(ref eng) = self.engine {
                    let manifest = eng.build_candidate_manifest(top_n);
                    let lines: Vec<String> = manifest.candidates.iter().map(|c| {
                        format!("  Rank {}: genome_id={}, fitness={:.4}", c.rank, c.genome_id.0, c.fitness.0)
                    }).collect();
                    apply_candidates(state, manifest);
                    let mut out = vec![format!("Top {} candidates:", top_n)];
                    out.extend(lines);
                    out
                } else {
                    apply_error(state, "No search active".to_string());
                    vec!["Error: No search active".to_string()]
                }
            }
            Action::Quit => vec!["Goodbye.".to_string()],
        }
    }
}
