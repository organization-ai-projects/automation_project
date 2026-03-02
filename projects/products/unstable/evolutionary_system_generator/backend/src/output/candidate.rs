use crate::evaluate::evaluation_report::EvaluationReport;
use crate::evaluate::fitness::Fitness;
use crate::genome::genome::Genome;
use crate::genome::genome_id::GenomeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub rank: usize,
    pub genome_id: GenomeId,
    pub genome: Genome,
    pub fitness: Fitness,
    pub report: EvaluationReport,
}
