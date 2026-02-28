use crate::constraints::constraint::Constraint;
use crate::constraints::constraint_engine::ConstraintEngine;
use crate::evaluate::evaluator::Evaluator;
use crate::genome::crossover::uniform_crossover;
use crate::genome::genome::{Genome, RuleEntry};
use crate::genome::genome_id::GenomeId;
use crate::genome::mutation::Mutation;
use crate::output::candidate::Candidate;
use crate::output::candidate_manifest::CandidateManifest;
use crate::output::manifest_hash::ManifestHash;
use crate::replay::event_log::EventLog;
use crate::replay::search_event::SearchEventKind;
use crate::search::population::{Individual, Population};
use crate::search::selection::tournament_select;
use crate::seed::seed::{Seed, Xorshift64};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub seed: Seed,
    pub population_size: usize,
    pub max_generations: u32,
    pub rule_pool: Vec<String>,
    pub constraints: Vec<Constraint>,
}

pub struct EvolutionEngine {
    config: SearchConfig,
    rng: Xorshift64,
    population: Population,
    event_log: EventLog,
    next_genome_id: u32,
    done: bool,
}

impl EvolutionEngine {
    pub fn new(config: SearchConfig) -> Self {
        let rng = Xorshift64::from_seed(&config.seed);
        let population = Population {
            generation: 0,
            individuals: Vec::new(),
        };
        let mut engine = Self {
            config,
            rng,
            population,
            event_log: EventLog::default(),
            next_genome_id: 0,
            done: false,
        };
        engine.event_log.push(SearchEventKind::SearchStarted {
            seed: engine.config.seed.0,
            population_size: engine.config.population_size,
            max_generations: engine.config.max_generations,
        });
        engine.init_population();
        engine
    }

    pub fn step_generation(&mut self) -> bool {
        if self.done {
            return true;
        }
        if self.population.generation >= self.config.max_generations {
            self.event_log.push(SearchEventKind::SearchComplete {
                total_generations: self.population.generation,
            });
            self.done = true;
            return true;
        }
        self.evolve_one_generation();
        if self.population.generation >= self.config.max_generations {
            self.event_log.push(SearchEventKind::SearchComplete {
                total_generations: self.population.generation,
            });
            self.done = true;
            return true;
        }
        false
    }

    pub fn run_to_end(&mut self) {
        while !self.step_generation() {}
    }

    pub fn get_population(&self) -> &Population {
        &self.population
    }

    pub fn get_event_log(&self) -> &EventLog {
        &self.event_log
    }

    pub fn build_candidate_manifest(&self, top_n: usize) -> CandidateManifest {
        let mut sorted = self.population.individuals.clone();
        sorted.sort_by(|a, b| {
            b.fitness
                .0
                .partial_cmp(&a.fitness.0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted.truncate(top_n);

        let candidates: Vec<Candidate> = sorted
            .into_iter()
            .enumerate()
            .map(|(rank, ind)| Candidate {
                rank,
                genome_id: ind.genome.id,
                genome: ind.genome.clone(),
                fitness: ind.fitness.clone(),
                report: ind.report.clone(),
            })
            .collect();

        let generation = self.population.generation;
        let seed = self.config.seed.0;
        let manifest_hash = ManifestHash::compute(&candidates, generation, seed);

        CandidateManifest {
            candidates,
            manifest_hash,
            generation,
            seed,
        }
    }

    fn alloc_genome_id(&mut self) -> GenomeId {
        let id = GenomeId(self.next_genome_id);
        self.next_genome_id += 1;
        id
    }

    fn create_initial_genome(&mut self, id: GenomeId) -> Genome {
        let rules: Vec<RuleEntry> = self
            .config
            .rule_pool
            .iter()
            .map(|name| RuleEntry {
                name: name.clone(),
                weight: self.rng.next_range(11) as u32,
            })
            .collect();
        Genome { id, rules }
    }

    fn init_population(&mut self) {
        let evaluator = Evaluator::new(ConstraintEngine::new(self.config.constraints.clone()));
        let mut individuals = Vec::with_capacity(self.config.population_size);
        for _ in 0..self.config.population_size {
            let id = self.alloc_genome_id();
            let genome = self.create_initial_genome(id);
            let weights: Vec<u32> = genome.rules.iter().map(|r| r.weight).collect();
            self.event_log.push(SearchEventKind::GenomeCreated {
                genome_id: id.0,
                weights,
            });
            let (fitness, report) = evaluator.evaluate(&genome);
            self.event_log.push(SearchEventKind::FitnessEvaluated {
                genome_id: id.0,
                fitness: fitness.0,
            });
            individuals.push(Individual {
                genome,
                fitness,
                report,
            });
        }
        self.population.individuals = individuals;

        let best = self
            .population
            .individuals
            .iter()
            .map(|i| i.fitness.0)
            .fold(f64::NEG_INFINITY, f64::max);
        self.event_log.push(SearchEventKind::GenerationComplete {
            generation: 0,
            best_fitness: best,
            population_size: self.population.individuals.len(),
        });
    }

    fn evolve_one_generation(&mut self) {
        let evaluator = Evaluator::new(ConstraintEngine::new(self.config.constraints.clone()));
        let pop_size = self.config.population_size;
        let mut new_individuals = Vec::with_capacity(pop_size);

        for _ in 0..pop_size {
            let use_crossover = self.rng.next_range(2) == 0;
            let new_genome = if use_crossover && self.population.individuals.len() >= 2 {
                let parent_a = tournament_select(&mut self.rng, &self.population)
                    .genome
                    .clone();
                let parent_b = tournament_select(&mut self.rng, &self.population)
                    .genome
                    .clone();
                let child_id = self.alloc_genome_id();
                let child = uniform_crossover(&mut self.rng, &parent_a, &parent_b, child_id);
                self.event_log.push(SearchEventKind::CrossoverApplied {
                    child_id: child_id.0,
                    parent_a_id: parent_a.id.0,
                    parent_b_id: parent_b.id.0,
                });
                child
            } else {
                let parent = tournament_select(&mut self.rng, &self.population)
                    .genome
                    .clone();
                let parent_id = parent.id;
                let child_id = self.alloc_genome_id();
                let mut child = parent;
                child.id = child_id;
                let mutation = Mutation::random(&mut self.rng, &child);
                mutation.apply(&mut child);
                self.event_log.push(SearchEventKind::MutationApplied {
                    genome_id: child_id.0,
                    mutation,
                    parent_id: parent_id.0,
                });
                child
            };

            let (fitness, report) = evaluator.evaluate(&new_genome);
            self.event_log.push(SearchEventKind::FitnessEvaluated {
                genome_id: new_genome.id.0,
                fitness: fitness.0,
            });
            new_individuals.push(Individual {
                genome: new_genome,
                fitness,
                report,
            });
        }

        self.population.individuals = new_individuals;
        self.population.generation += 1;

        let best = self
            .population
            .individuals
            .iter()
            .map(|i| i.fitness.0)
            .fold(f64::NEG_INFINITY, f64::max);
        self.event_log.push(SearchEventKind::GenerationComplete {
            generation: self.population.generation,
            best_fitness: best,
            population_size: self.population.individuals.len(),
        });
    }
}
