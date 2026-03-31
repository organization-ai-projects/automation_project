use crate::domain::prediction_confidence::PredictionConfidence;
use crate::domain::raw_prediction::RawPrediction;
use crate::domain::weather_state::WeatherState;
use crate::simulation::constraint_engine::ConstraintEngine;

fn make_state(
    pressure_trend: f64,
    humidity: f64,
    wind: f64,
    cloud: f64,
    instability: f64,
) -> WeatherState {
    WeatherState {
        pressure_hpa: 1013.25,
        pressure_trend,
        temperature_c: 20.0,
        temperature_trend: 0.0,
        humidity_pct: humidity,
        humidity_trend: 0.0,
        wind_speed_kmh: wind,
        wind_trend: 0.0,
        cloudiness_pct: cloud,
        precipitation_likelihood: 0.3,
        storm_likelihood: 0.2,
        instability_index: instability,
        confidence: 0.8,
    }
}

fn make_prediction(clear: f64, precip: f64, storm: f64, calm: f64, windy: f64) -> RawPrediction {
    RawPrediction {
        forecast_label: "Test".to_string(),
        confidence: PredictionConfidence {
            clear_sky: clear,
            precipitation: precip,
            storm,
            calm,
            windy,
        },
        rationale: "test".to_string(),
    }
}

#[test]
fn pressure_drop_clear_sky_triggers() {
    let state = make_state(-5.0, 70.0, 10.0, 50.0, 0.2);
    let prediction = make_prediction(0.8, 0.3, 0.1, 0.5, 0.1);
    let result = ConstraintEngine::validate(&state, &prediction);
    assert!(!result.is_coherent);
    assert!(
        result
            .violations
            .iter()
            .any(|v| v.rule.id() == "PRESSURE_DROP_CLEAR_SKY")
    );
}

#[test]
fn low_humidity_precipitation_triggers() {
    let state = make_state(0.0, 20.0, 10.0, 50.0, 0.2);
    let prediction = make_prediction(0.5, 0.7, 0.1, 0.5, 0.1);
    let result = ConstraintEngine::validate(&state, &prediction);
    assert!(!result.is_coherent);
    assert!(
        result
            .violations
            .iter()
            .any(|v| v.rule.id() == "LOW_HUMIDITY_PRECIPITATION")
    );
}

#[test]
fn low_cloud_storm_triggers() {
    let state = make_state(0.0, 50.0, 10.0, 20.0, 0.1);
    let prediction = make_prediction(0.5, 0.2, 0.5, 0.5, 0.1);
    let result = ConstraintEngine::validate(&state, &prediction);
    assert!(!result.is_coherent);
    assert!(
        result
            .violations
            .iter()
            .any(|v| v.rule.id() == "LOW_CLOUD_STORM")
    );
}

#[test]
fn high_wind_calm_triggers() {
    let state = make_state(0.0, 50.0, 50.0, 50.0, 0.2);
    let prediction = make_prediction(0.5, 0.2, 0.1, 0.5, 0.3);
    let result = ConstraintEngine::validate(&state, &prediction);
    assert!(!result.is_coherent);
    assert!(
        result
            .violations
            .iter()
            .any(|v| v.rule.id() == "HIGH_WIND_CALM")
    );
}

#[test]
fn instability_coherence_triggers() {
    let state = make_state(0.0, 50.0, 10.0, 50.0, 0.8);
    let prediction = make_prediction(0.5, 0.2, 0.1, 0.5, 0.1);
    let result = ConstraintEngine::validate(&state, &prediction);
    assert!(!result.is_coherent);
    assert!(
        result
            .violations
            .iter()
            .any(|v| v.rule.id() == "INSTABILITY_COHERENCE")
    );
}

#[test]
fn mutual_incoherence_triggers() {
    let state = make_state(0.0, 50.0, 10.0, 50.0, 0.2);
    let prediction = make_prediction(0.9, 0.8, 0.7, 0.8, 0.9);
    let result = ConstraintEngine::validate(&state, &prediction);
    assert!(!result.is_coherent);
    assert!(
        result
            .violations
            .iter()
            .any(|v| v.rule.id() == "MUTUAL_INCOHERENCE")
    );
}

#[test]
fn coherent_prediction_passes_validation() {
    let state = make_state(0.0, 50.0, 10.0, 50.0, 0.2);
    let prediction = make_prediction(0.3, 0.3, 0.1, 0.3, 0.1);
    let result = ConstraintEngine::validate(&state, &prediction);
    assert!(result.is_coherent);
    assert!(result.violations.is_empty());
}

#[test]
fn constraint_evaluation_ordering_is_deterministic() {
    let state = make_state(-5.0, 20.0, 50.0, 20.0, 0.8);
    let prediction = make_prediction(0.9, 0.8, 0.7, 0.8, 0.9);

    let result1 = ConstraintEngine::validate(&state, &prediction);
    let result2 = ConstraintEngine::validate(&state, &prediction);

    assert_eq!(result1.violations.len(), result2.violations.len());
    for (a, b) in result1.violations.iter().zip(result2.violations.iter()) {
        assert_eq!(a.rule, b.rule);
    }
}
