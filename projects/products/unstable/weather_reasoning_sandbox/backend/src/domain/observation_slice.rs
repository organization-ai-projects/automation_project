use serde::{Deserialize, Serialize};

use crate::domain::observation_record::ObservationRecord;
use crate::domain::tick_index::TickIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservationSlice {
    pub tick: TickIndex,
    pub records: Vec<ObservationRecord>,
}
