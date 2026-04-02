use crate::domain::prediction_confidence::PredictionConfidence;
use crate::domain::raw_prediction::RawPrediction;
use crate::domain::weather_state::WeatherState;

pub struct RawPredictionEngine;

impl RawPredictionEngine {
    pub fn predict(state: &WeatherState, seed: u64, tick: u64) -> RawPrediction {
        let deterministic_noise = Self::deterministic_noise(seed, tick);

        let clear_sky = Self::compute_clear_sky(state, deterministic_noise);
        let precipitation = Self::compute_precipitation(state, deterministic_noise);
        let storm = Self::compute_storm(state, deterministic_noise);
        let calm = Self::compute_calm(state, deterministic_noise);
        let windy = Self::compute_windy(state, deterministic_noise);

        let confidence = PredictionConfidence {
            clear_sky,
            precipitation,
            storm,
            calm,
            windy,
        };

        let forecast_label = Self::select_label(&confidence);
        let rationale = Self::build_rationale(state, &confidence);

        RawPrediction {
            forecast_label,
            confidence,
            rationale,
        }
    }

    fn deterministic_noise(seed: u64, tick: u64) -> f64 {
        let combined = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(tick.wrapping_mul(1442695040888963407));
        let bits = (combined >> 33) ^ combined;
        let normalized = (bits % 1000) as f64 / 1000.0;
        (normalized - 0.5) * 0.1
    }

    fn compute_clear_sky(state: &WeatherState, noise: f64) -> f64 {
        let base = 1.0 - (state.cloudiness_pct / 100.0);
        let humidity_penalty = if state.humidity_pct > 70.0 { 0.1 } else { 0.0 };
        (base - humidity_penalty + noise).clamp(0.0, 1.0)
    }

    fn compute_precipitation(state: &WeatherState, noise: f64) -> f64 {
        let base = state.precipitation_likelihood;
        (base + noise * 0.5).clamp(0.0, 1.0)
    }

    fn compute_storm(state: &WeatherState, noise: f64) -> f64 {
        let base = state.storm_likelihood;
        (base + noise * 0.3).clamp(0.0, 1.0)
    }

    fn compute_calm(state: &WeatherState, noise: f64) -> f64 {
        let base = if state.wind_speed_kmh < 15.0 {
            0.7
        } else if state.wind_speed_kmh < 30.0 {
            0.4
        } else {
            0.1
        };
        (base + noise).clamp(0.0, 1.0)
    }

    fn compute_windy(state: &WeatherState, noise: f64) -> f64 {
        let base = (state.wind_speed_kmh / 80.0).min(1.0);
        (base + noise).clamp(0.0, 1.0)
    }

    fn select_label(conf: &PredictionConfidence) -> String {
        let candidates = [
            ("Clear", conf.clear_sky),
            ("Precipitation", conf.precipitation),
            ("Storm", conf.storm),
            ("Calm", conf.calm),
            ("Windy", conf.windy),
        ];
        candidates
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(label, _)| label.to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    }

    fn build_rationale(state: &WeatherState, conf: &PredictionConfidence) -> String {
        format!(
            "Based on pressure={:.1}hPa(trend={:.2}),humidity={:.1}%,wind={:.1}km/h,cloud={:.1}%,instability={:.3}. Top confidence: clear={:.3},precip={:.3},storm={:.3},calm={:.3},windy={:.3}",
            state.pressure_hpa,
            state.pressure_trend,
            state.humidity_pct,
            state.wind_speed_kmh,
            state.cloudiness_pct,
            state.instability_index,
            conf.clear_sky,
            conf.precipitation,
            conf.storm,
            conf.calm,
            conf.windy,
        )
    }
}
