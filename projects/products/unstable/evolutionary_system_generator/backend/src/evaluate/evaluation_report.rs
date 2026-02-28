use serde::{Deserialize, Serialize};
use crate::evaluate::fitness::Fitness;
use crate::genome::genome_id::GenomeId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationReport {
    pub genome_id: GenomeId,
    pub fitness: Fitness,
    pub active_rule_count: usize,
    pub total_weight: u32,
    pub constraint_violations: Vec<String>,
    pub satisfied_constraints: bool,
}
