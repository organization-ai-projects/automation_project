use serde::{Deserialize, Serialize};

use crate::domain::checksum_value::ChecksumValue;
use crate::domain::corrected_prediction::CorrectedPrediction;
use crate::domain::constraint_violation::ConstraintViolation;
use crate::domain::correction_action::CorrectionAction;
use crate::domain::raw_prediction::RawPrediction;
use crate::domain::run_metadata::RunMetadata;
use crate::domain::tick_index::TickIndex;
use crate::domain::weather_state::WeatherState;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TickReport {
    pub tick: TickIndex,
    pub weather_state: WeatherState,
    pub raw_prediction: RawPrediction,
    pub violations: Vec<ConstraintViolation>,
    pub corrections: Vec<CorrectionAction>,
    pub corrected_prediction: CorrectedPrediction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportModel {
    pub metadata: RunMetadata,
    pub tick_reports: Vec<TickReport>,
    pub contradiction_count: usize,
    pub total_violations: usize,
    pub total_corrections: usize,
    pub final_corrected_prediction: Option<CorrectedPrediction>,
    pub report_checksum: ChecksumValue,
    pub snapshot_checksum: Option<ChecksumValue>,
    pub replay_equivalence: Option<bool>,
}
