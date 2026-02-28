use serde::{Deserialize, Serialize};
use crate::time::turn::Turn;
use super::resolution_step::ResolutionStep;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdjudicationReport {
    pub turn: Turn,
    pub steps: Vec<ResolutionStep>,
}
