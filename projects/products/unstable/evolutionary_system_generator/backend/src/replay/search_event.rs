use serde::{Deserialize, Serialize};
use crate::genome::mutation::Mutation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchEventKind {
    SearchStarted { seed: u64, population_size: usize, max_generations: u32 },
    GenomeCreated { genome_id: u32, weights: Vec<u32> },
    MutationApplied { genome_id: u32, mutation: Mutation, parent_id: u32 },
    CrossoverApplied { child_id: u32, parent_a_id: u32, parent_b_id: u32 },
    FitnessEvaluated { genome_id: u32, fitness: f64 },
    GenerationComplete { generation: u32, best_fitness: f64, population_size: usize },
    SearchComplete { total_generations: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEvent {
    pub sequence: u64,
    pub kind: SearchEventKind,
}
