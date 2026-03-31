use crate::domain::contradiction_entry::ContradictionEntry;
use crate::domain::contradiction_memory::ContradictionMemory;
use crate::domain::corrected_prediction::CorrectedPrediction;
use crate::domain::correction_action::CorrectionAction;
use crate::domain::constraint_violation::ConstraintViolation;
use crate::domain::raw_prediction::RawPrediction;
use crate::domain::tick_index::TickIndex;

pub struct ContradictionRecorder;

impl ContradictionRecorder {
    pub fn record(
        memory: &mut ContradictionMemory,
        tick: TickIndex,
        raw_prediction: &RawPrediction,
        violations: &[ConstraintViolation],
        corrections: &[CorrectionAction],
        corrected_prediction: &CorrectedPrediction,
    ) {
        if !violations.is_empty() {
            let entry = ContradictionEntry {
                tick,
                raw_prediction: raw_prediction.clone(),
                violations: violations.to_vec(),
                corrections: corrections.to_vec(),
                corrected_prediction: corrected_prediction.clone(),
            };
            memory.append(entry);
        }
    }
}
