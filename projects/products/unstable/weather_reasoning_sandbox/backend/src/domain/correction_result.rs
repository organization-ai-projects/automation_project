use serde::{Deserialize, Serialize};

use crate::domain::corrected_prediction::CorrectedPrediction;
use crate::domain::correction_action::CorrectionAction;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorrectionResult {
    pub actions: Vec<CorrectionAction>,
    pub corrected: CorrectedPrediction,
}
