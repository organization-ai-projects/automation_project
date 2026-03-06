// projects/products/unstable/evolutionary_system_generator/backend/src/output/candidate.rs
use crate::evaluate::evaluation_report::EvaluationReport;
use crate::evaluate::fitness::Fitness;
use crate::genetics::genome::Genome;
use crate::genetics::genome_id::GenomeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub rank: usize,
    pub genome_id: GenomeId,
    pub genome: Genome,
    pub fitness: Fitness,
    pub report: EvaluationReport,
}
