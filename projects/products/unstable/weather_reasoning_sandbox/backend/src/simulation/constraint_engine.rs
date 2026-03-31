use crate::domain::constraint_rule::ConstraintRule;
use crate::domain::constraint_violation::ConstraintViolation;
use crate::domain::raw_prediction::RawPrediction;
use crate::domain::validation_result::ValidationResult;
use crate::domain::weather_state::WeatherState;

pub struct ConstraintEngine;

impl ConstraintEngine {
    pub fn validate(state: &WeatherState, prediction: &RawPrediction) -> ValidationResult {
        let mut violations = Vec::new();

        for rule in ConstraintRule::all() {
            if let Some(violation) = Self::evaluate_rule(rule, state, prediction) {
                violations.push(violation);
            }
        }

        violations.sort_by(|a, b| a.rule.cmp(&b.rule));
        ValidationResult::with_violations(violations)
    }

    fn evaluate_rule(
        rule: &ConstraintRule,
        state: &WeatherState,
        prediction: &RawPrediction,
    ) -> Option<ConstraintViolation> {
        match rule {
            ConstraintRule::PressureDropClearSky => {
                if state.pressure_trend < -3.0
                    && state.humidity_pct > 60.0
                    && prediction.confidence.clear_sky > 0.5
                {
                    Some(ConstraintViolation {
                        rule: rule.clone(),
                        description: rule.description().to_string(),
                        reason: format!(
                            "Pressure dropping {:.1}hPa with humidity {:.1}% but clear_sky confidence is {:.3}",
                            state.pressure_trend,
                            state.humidity_pct,
                            prediction.confidence.clear_sky
                        ),
                        severity: prediction.confidence.clear_sky - 0.3,
                    })
                } else {
                    None
                }
            }
            ConstraintRule::LowHumidityPrecipitation => {
                if state.humidity_pct < 30.0 && prediction.confidence.precipitation > 0.4 {
                    Some(ConstraintViolation {
                        rule: rule.clone(),
                        description: rule.description().to_string(),
                        reason: format!(
                            "Humidity at {:.1}% is too low for precipitation confidence of {:.3}",
                            state.humidity_pct, prediction.confidence.precipitation
                        ),
                        severity: prediction.confidence.precipitation - 0.1,
                    })
                } else {
                    None
                }
            }
            ConstraintRule::LowCloudStorm => {
                if state.cloudiness_pct < 30.0
                    && state.instability_index < 0.3
                    && prediction.confidence.storm > 0.3
                {
                    Some(ConstraintViolation {
                        rule: rule.clone(),
                        description: rule.description().to_string(),
                        reason: format!(
                            "Cloudiness {:.1}% and instability {:.3} too low for storm confidence {:.3}",
                            state.cloudiness_pct,
                            state.instability_index,
                            prediction.confidence.storm
                        ),
                        severity: prediction.confidence.storm - 0.1,
                    })
                } else {
                    None
                }
            }
            ConstraintRule::HighWindCalm => {
                if state.wind_speed_kmh > 40.0 && prediction.confidence.calm > 0.3 {
                    Some(ConstraintViolation {
                        rule: rule.clone(),
                        description: rule.description().to_string(),
                        reason: format!(
                            "Wind at {:.1}km/h contradicts calm confidence of {:.3}",
                            state.wind_speed_kmh, prediction.confidence.calm
                        ),
                        severity: prediction.confidence.calm,
                    })
                } else {
                    None
                }
            }
            ConstraintRule::InstabilityCoherence => {
                if state.instability_index > 0.6 && prediction.confidence.storm < 0.2 {
                    Some(ConstraintViolation {
                        rule: rule.clone(),
                        description: rule.description().to_string(),
                        reason: format!(
                            "High instability {:.3} but storm confidence only {:.3}",
                            state.instability_index, prediction.confidence.storm
                        ),
                        severity: 0.5,
                    })
                } else {
                    None
                }
            }
            ConstraintRule::MutualIncoherence => {
                let sum = prediction.confidence.clear_sky
                    + prediction.confidence.precipitation
                    + prediction.confidence.storm
                    + prediction.confidence.calm
                    + prediction.confidence.windy;
                if sum > 3.0 {
                    Some(ConstraintViolation {
                        rule: rule.clone(),
                        description: rule.description().to_string(),
                        reason: format!(
                            "Total confidence sum {:.3} exceeds coherence threshold 3.0",
                            sum
                        ),
                        severity: sum - 3.0,
                    })
                } else {
                    None
                }
            }
        }
    }
}
