use crate::domain::prediction_confidence::PredictionConfidence;
use crate::domain::raw_prediction::RawPrediction;
use crate::domain::weather_state::WeatherState;
use crate::simulation::constraint_engine::ConstraintEngine;
use crate::simulation::correction_engine::CorrectionEngine;

fn make_state_for_correction(
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

fn make_prediction_for_correction(
    clear: f64,
    precip: f64,
    storm: f64,
    calm: f64,
    windy: f64,
) -> RawPrediction {
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
fn single_violation_correction() {
    let state = make_state_for_correction(-5.0, 70.0, 10.0, 50.0, 0.2);
    let prediction = make_prediction_for_correction(0.8, 0.3, 0.1, 0.5, 0.1);
    let validation = ConstraintEngine::validate(&state, &prediction);
    let result = CorrectionEngine::correct(&prediction, &validation);

    assert!(!result.actions.is_empty());
    assert!(result.corrected.confidence.clear_sky < prediction.confidence.clear_sky);
}

#[test]
fn multiple_violation_correction() {
    let state = make_state_for_correction(-5.0, 20.0, 50.0, 20.0, 0.8);
    let prediction = make_prediction_for_correction(0.9, 0.8, 0.5, 0.8, 0.9);
    let validation = ConstraintEngine::validate(&state, &prediction);
    let result = CorrectionEngine::correct(&prediction, &validation);

    assert!(result.actions.len() > 1);
    assert!(!result.corrected.corrections_applied.is_empty());
    assert!(!result.corrected.explanation.is_empty());
}

#[test]
fn correction_ordering_is_deterministic() {
    let state = make_state_for_correction(-5.0, 20.0, 50.0, 20.0, 0.8);
    let prediction = make_prediction_for_correction(0.9, 0.8, 0.5, 0.8, 0.9);
    let validation = ConstraintEngine::validate(&state, &prediction);

    let result1 = CorrectionEngine::correct(&prediction, &validation);
    let result2 = CorrectionEngine::correct(&prediction, &validation);

    assert_eq!(result1.actions.len(), result2.actions.len());
    for (a, b) in result1.actions.iter().zip(result2.actions.iter()) {
        assert_eq!(a.field, b.field);
        assert_eq!(a.original_value, b.original_value);
        assert_eq!(a.corrected_value, b.corrected_value);
    }
    assert_eq!(
        result1.corrected.canonical_string(),
        result2.corrected.canonical_string()
    );
}

#[test]
fn coherent_prediction_has_no_corrections() {
    let state = make_state_for_correction(0.0, 50.0, 10.0, 50.0, 0.2);
    let prediction = make_prediction_for_correction(0.3, 0.3, 0.1, 0.3, 0.1);
    let validation = ConstraintEngine::validate(&state, &prediction);
    let result = CorrectionEngine::correct(&prediction, &validation);

    assert!(result.actions.is_empty());
    assert_eq!(
        result.corrected.confidence.clear_sky,
        prediction.confidence.clear_sky
    );
}

#[test]
fn contradiction_memory_records_violations() {
    let state = make_state_for_correction(-5.0, 70.0, 10.0, 50.0, 0.2);
    let prediction = make_prediction_for_correction(0.8, 0.3, 0.1, 0.5, 0.1);
    let validation = ConstraintEngine::validate(&state, &prediction);
    let correction_result = CorrectionEngine::correct(&prediction, &validation);

    let mut memory = crate::domain::contradiction_memory::ContradictionMemory::new();
    crate::simulation::contradiction_recorder::ContradictionRecorder::record(
        &mut memory,
        crate::domain::tick_index::TickIndex(0),
        &prediction,
        &validation.violations,
        &correction_result.actions,
        &correction_result.corrected,
    );

    assert_eq!(memory.len(), 1);
    assert!(!memory.entries()[0].violations.is_empty());
}
