use crate::solve::solve_step::SolveStep;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SolveReport {
    pub steps: Vec<SolveStep>,
}
