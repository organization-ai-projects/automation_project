use serde::{Deserialize, Serialize};

use crate::domain::constraint_violation::ConstraintViolation;
use crate::domain::corrected_prediction::CorrectedPrediction;
use crate::domain::correction_action::CorrectionAction;
use crate::domain::observation_record::ObservationRecord;
use crate::domain::raw_prediction::RawPrediction;
use crate::domain::run_metadata::RunMetadata;
use crate::domain::tick_index::TickIndex;
use crate::domain::weather_state::WeatherState;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JournalEvent {
    RunStarted {
        metadata: RunMetadata,
    },
    TickStarted {
        tick: TickIndex,
    },
    ObservationsLoaded {
        tick: TickIndex,
        records: Vec<ObservationRecord>,
    },
    StateUpdated {
        tick: TickIndex,
        state: WeatherState,
    },
    RawPredictionGenerated {
        tick: TickIndex,
        prediction: RawPrediction,
    },
    ConstraintsEvaluated {
        tick: TickIndex,
        violations: Vec<ConstraintViolation>,
    },
    CorrectionsApplied {
        tick: TickIndex,
        actions: Vec<CorrectionAction>,
    },
    CorrectedPredictionEmitted {
        tick: TickIndex,
        prediction: CorrectedPrediction,
    },
    TickCompleted {
        tick: TickIndex,
    },
    RunCompleted,
}
