use serde::{Deserialize, Serialize};
use crate::evaluate::evaluation_report::EvaluationReport;
use crate::evaluate::fitness::Fitness;
use crate::genome::genome::Genome;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Individual {
    pub genome: Genome,
    pub fitness: Fitness,
    pub report: EvaluationReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    pub generation: u32,
    pub individuals: Vec<Individual>,
}
