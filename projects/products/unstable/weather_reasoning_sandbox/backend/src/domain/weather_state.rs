use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherState {
    pub pressure_hpa: f64,
    pub pressure_trend: f64,
    pub temperature_c: f64,
    pub temperature_trend: f64,
    pub humidity_pct: f64,
    pub humidity_trend: f64,
    pub wind_speed_kmh: f64,
    pub wind_trend: f64,
    pub cloudiness_pct: f64,
    pub precipitation_likelihood: f64,
    pub storm_likelihood: f64,
    pub instability_index: f64,
    pub confidence: f64,
}

impl WeatherState {
    pub fn initial() -> Self {
        Self {
            pressure_hpa: 1013.25,
            pressure_trend: 0.0,
            temperature_c: 20.0,
            temperature_trend: 0.0,
            humidity_pct: 50.0,
            humidity_trend: 0.0,
            wind_speed_kmh: 10.0,
            wind_trend: 0.0,
            cloudiness_pct: 30.0,
            precipitation_likelihood: 0.1,
            storm_likelihood: 0.05,
            instability_index: 0.1,
            confidence: 0.8,
        }
    }

    pub fn canonical_string(&self) -> String {
        format!(
            "p={:.2},pt={:.4},t={:.2},tt={:.4},h={:.2},ht={:.4},w={:.2},wt={:.4},c={:.2},pl={:.4},sl={:.4},ii={:.4},conf={:.4}",
            self.pressure_hpa,
            self.pressure_trend,
            self.temperature_c,
            self.temperature_trend,
            self.humidity_pct,
            self.humidity_trend,
            self.wind_speed_kmh,
            self.wind_trend,
            self.cloudiness_pct,
            self.precipitation_likelihood,
            self.storm_likelihood,
            self.instability_index,
            self.confidence,
        )
    }
}
