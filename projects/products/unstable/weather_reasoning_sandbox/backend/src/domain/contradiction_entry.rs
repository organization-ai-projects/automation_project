use serde::{Deserialize, Serialize};

use crate::domain::corrected_prediction::CorrectedPrediction;
use crate::domain::constraint_violation::ConstraintViolation;
use crate::domain::correction_action::CorrectionAction;
use crate::domain::raw_prediction::RawPrediction;
use crate::domain::tick_index::TickIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContradictionEntry {
    pub tick: TickIndex,
    pub raw_prediction: RawPrediction,
    pub violations: Vec<ConstraintViolation>,
    pub corrections: Vec<CorrectionAction>,
    pub corrected_prediction: CorrectedPrediction,
}
