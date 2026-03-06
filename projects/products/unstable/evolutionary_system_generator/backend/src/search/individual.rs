// projects/products/unstable/evolutionary_system_generator/backend/src/search/individual.rs
use crate::evaluate::evaluation_report::EvaluationReport;
use crate::evaluate::fitness::Fitness;
use crate::genetics::genome::Genome;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Individual {
    pub genome: Genome,
    pub fitness: Fitness,
    pub report: EvaluationReport,
}
