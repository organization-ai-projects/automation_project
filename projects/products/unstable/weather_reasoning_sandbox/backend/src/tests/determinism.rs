use crate::domain::observation_record::ObservationRecord;
use crate::domain::weather_state::WeatherState;
use crate::infrastructure::dataset_loader::DatasetLoader;
use crate::infrastructure::dataset_parser::DatasetParser;
use crate::simulation::raw_prediction_engine::RawPredictionEngine;
use crate::simulation::simulation_engine::SimulationEngine;
use crate::simulation::state_transition::StateTransition;

fn sample_observations() -> Vec<ObservationRecord> {
    vec![
        ObservationRecord {
            timestamp_index: 0,
            pressure_hpa: 1010.0,
            temperature_c: 22.0,
            humidity_pct: 65.0,
            wind_speed_kmh: 15.0,
            cloudiness_pct: 40.0,
            precipitation_mm: 2.0,
        },
        ObservationRecord {
            timestamp_index: 1,
            pressure_hpa: 1008.0,
            temperature_c: 21.0,
            humidity_pct: 70.0,
            wind_speed_kmh: 20.0,
            cloudiness_pct: 55.0,
            precipitation_mm: 5.0,
        },
        ObservationRecord {
            timestamp_index: 2,
            pressure_hpa: 1005.0,
            temperature_c: 19.0,
            humidity_pct: 80.0,
            wind_speed_kmh: 30.0,
            cloudiness_pct: 75.0,
            precipitation_mm: 10.0,
        },
    ]
}

#[test]
fn state_transition_is_deterministic() {
    let prior = WeatherState::initial();
    let obs = sample_observations();
    let state1 = StateTransition::derive(&prior, &obs);
    let state2 = StateTransition::derive(&prior, &obs);
    assert_eq!(state1.canonical_string(), state2.canonical_string());
}

#[test]
fn raw_prediction_is_deterministic() {
    let state = WeatherState::initial();
    let pred1 = RawPredictionEngine::predict(&state, 42, 0);
    let pred2 = RawPredictionEngine::predict(&state, 42, 0);
    assert_eq!(pred1.canonical_string(), pred2.canonical_string());
}

#[test]
fn different_seeds_produce_different_predictions() {
    let state = WeatherState::initial();
    let pred1 = RawPredictionEngine::predict(&state, 1, 0);
    let pred2 = RawPredictionEngine::predict(&state, 999, 0);
    assert_ne!(pred1.canonical_string(), pred2.canonical_string());
}

#[test]
fn observation_ordering_is_deterministic() {
    let (_, obs1) = DatasetLoader::load_default(42, 5);
    let (_, obs2) = DatasetLoader::load_default(42, 5);
    assert_eq!(obs1.len(), obs2.len());
    for (a, b) in obs1.iter().zip(obs2.iter()) {
        assert_eq!(a.timestamp_index, b.timestamp_index);
        assert_eq!(a.pressure_hpa, b.pressure_hpa);
    }
}

#[test]
fn dataset_checksum_is_stable() {
    let (_, obs) = DatasetLoader::load_default(42, 5);
    let cs1 = DatasetParser::compute_checksum(&obs);
    let cs2 = DatasetParser::compute_checksum(&obs);
    assert_eq!(cs1, cs2);
}

#[test]
fn simulation_is_deterministic() {
    let (id1, obs1) = DatasetLoader::load_default(42, 5);
    let cs1 = DatasetParser::compute_checksum(&obs1);
    let out1 = SimulationEngine::run(42, 5, id1, cs1, obs1);

    let (id2, obs2) = DatasetLoader::load_default(42, 5);
    let cs2 = DatasetParser::compute_checksum(&obs2);
    let out2 = SimulationEngine::run(42, 5, id2, cs2, obs2);

    assert_eq!(out1.tick_reports.len(), out2.tick_reports.len());
    assert_eq!(
        out1.final_state.canonical_string(),
        out2.final_state.canonical_string()
    );
    assert_eq!(
        out1.contradiction_memory.len(),
        out2.contradiction_memory.len()
    );
}

#[test]
fn different_seeds_yield_different_simulation_outputs() {
    let (id1, obs1) = DatasetLoader::load_default(1, 5);
    let cs1 = DatasetParser::compute_checksum(&obs1);
    let out1 = SimulationEngine::run(1, 5, id1, cs1, obs1);

    let (id2, obs2) = DatasetLoader::load_default(999, 5);
    let cs2 = DatasetParser::compute_checksum(&obs2);
    let out2 = SimulationEngine::run(999, 5, id2, cs2, obs2);

    assert_ne!(
        out1.final_state.canonical_string(),
        out2.final_state.canonical_string()
    );
}
