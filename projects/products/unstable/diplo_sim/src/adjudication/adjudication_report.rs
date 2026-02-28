use super::resolution_step::ResolutionStep;
use crate::time::turn::Turn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdjudicationReport {
    pub turn: Turn,
    pub steps: Vec<ResolutionStep>,
}
