use serde::{Deserialize, Serialize};

use crate::domain::checksum_value::ChecksumValue;
use crate::domain::corrected_prediction::CorrectedPrediction;
use crate::domain::run_metadata::RunMetadata;
use crate::domain::weather_state::WeatherState;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotModel {
    pub metadata: RunMetadata,
    pub final_weather_state: WeatherState,
    pub contradiction_count: usize,
    pub journal_event_count: usize,
    pub final_corrected_prediction: Option<CorrectedPrediction>,
    pub snapshot_checksum: ChecksumValue,
}
