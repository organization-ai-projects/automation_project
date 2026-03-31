use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservationRecord {
    pub timestamp_index: u64,
    pub pressure_hpa: f64,
    pub temperature_c: f64,
    pub humidity_pct: f64,
    pub wind_speed_kmh: f64,
    pub cloudiness_pct: f64,
    pub precipitation_mm: f64,
}
