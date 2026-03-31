use crate::domain::constraint_rule::ConstraintRule;
use crate::domain::constraint_violation::ConstraintViolation;
use crate::domain::corrected_prediction::CorrectedPrediction;
use crate::domain::correction_action::CorrectionAction;
use crate::domain::correction_result::CorrectionResult;
use crate::domain::prediction_confidence::PredictionConfidence;
use crate::domain::raw_prediction::RawPrediction;
use crate::domain::validation_result::ValidationResult;

pub struct CorrectionEngine;

impl CorrectionEngine {
    pub fn correct(prediction: &RawPrediction, validation: &ValidationResult) -> CorrectionResult {
        let mut confidence = prediction.confidence.clone();
        let mut actions = Vec::new();

        let mut sorted_violations = validation.violations.clone();
        sorted_violations.sort_by(|a, b| a.rule.cmp(&b.rule));

        for violation in &sorted_violations {
            let new_actions = Self::apply_correction(violation, &mut confidence);
            actions.extend(new_actions);
        }

        let corrections_applied: Vec<String> = actions
            .iter()
            .map(|a| {
                format!(
                    "{}: {} {:.3}->{:.3}",
                    a.triggered_by, a.field, a.original_value, a.corrected_value
                )
            })
            .collect();

        let explanation = if actions.is_empty() {
            "No corrections needed; prediction is coherent.".to_string()
        } else {
            let action_descs: Vec<String> = actions.iter().map(|a| a.reason.clone()).collect();
            format!(
                "Applied {} corrections: {}",
                actions.len(),
                action_descs.join("; ")
            )
        };

        let forecast_label = Self::select_label(&confidence);

        let corrected = CorrectedPrediction {
            forecast_label,
            confidence,
            corrections_applied,
            explanation,
        };

        CorrectionResult { actions, corrected }
    }

    fn apply_correction(
        violation: &ConstraintViolation,
        confidence: &mut PredictionConfidence,
    ) -> Vec<CorrectionAction> {
        let mut actions = Vec::new();

        match &violation.rule {
            ConstraintRule::PressureDropClearSky => {
                let original = confidence.clear_sky;
                confidence.clear_sky =
                    (confidence.clear_sky - violation.severity).clamp(0.0, 1.0);
                actions.push(CorrectionAction {
                    triggered_by: violation.rule.clone(),
                    field: "clear_sky".to_string(),
                    original_value: original,
                    corrected_value: confidence.clear_sky,
                    reason: format!(
                        "Reduced clear_sky from {:.3} to {:.3} due to pressure drop",
                        original, confidence.clear_sky
                    ),
                });
            }
            ConstraintRule::LowHumidityPrecipitation => {
                let original = confidence.precipitation;
                confidence.precipitation =
                    (confidence.precipitation - violation.severity).clamp(0.0, 1.0);
                actions.push(CorrectionAction {
                    triggered_by: violation.rule.clone(),
                    field: "precipitation".to_string(),
                    original_value: original,
                    corrected_value: confidence.precipitation,
                    reason: format!(
                        "Reduced precipitation from {:.3} to {:.3} due to low humidity",
                        original, confidence.precipitation
                    ),
                });
            }
            ConstraintRule::LowCloudStorm => {
                let original = confidence.storm;
                confidence.storm = (confidence.storm - violation.severity).clamp(0.0, 1.0);
                actions.push(CorrectionAction {
                    triggered_by: violation.rule.clone(),
                    field: "storm".to_string(),
                    original_value: original,
                    corrected_value: confidence.storm,
                    reason: format!(
                        "Reduced storm from {:.3} to {:.3} due to low cloud cover",
                        original, confidence.storm
                    ),
                });
            }
            ConstraintRule::HighWindCalm => {
                let original = confidence.calm;
                confidence.calm = (confidence.calm - violation.severity).clamp(0.0, 1.0);
                actions.push(CorrectionAction {
                    triggered_by: violation.rule.clone(),
                    field: "calm".to_string(),
                    original_value: original,
                    corrected_value: confidence.calm,
                    reason: format!(
                        "Reduced calm from {:.3} to {:.3} due to high wind",
                        original, confidence.calm
                    ),
                });
            }
            ConstraintRule::InstabilityCoherence => {
                let original = confidence.storm;
                confidence.storm = (confidence.storm + 0.3).clamp(0.0, 1.0);
                actions.push(CorrectionAction {
                    triggered_by: violation.rule.clone(),
                    field: "storm".to_string(),
                    original_value: original,
                    corrected_value: confidence.storm,
                    reason: format!(
                        "Increased storm from {:.3} to {:.3} due to high instability",
                        original, confidence.storm
                    ),
                });
            }
            ConstraintRule::MutualIncoherence => {
                let sum = confidence.clear_sky
                    + confidence.precipitation
                    + confidence.storm
                    + confidence.calm
                    + confidence.windy;
                if sum > 3.0 {
                    let factor = 3.0 / sum;
                    let orig_clear = confidence.clear_sky;
                    let orig_precip = confidence.precipitation;
                    let orig_storm = confidence.storm;
                    let orig_calm = confidence.calm;
                    let orig_windy = confidence.windy;

                    confidence.clear_sky *= factor;
                    confidence.precipitation *= factor;
                    confidence.storm *= factor;
                    confidence.calm *= factor;
                    confidence.windy *= factor;

                    actions.push(CorrectionAction {
                        triggered_by: violation.rule.clone(),
                        field: "clear_sky".to_string(),
                        original_value: orig_clear,
                        corrected_value: confidence.clear_sky,
                        reason: format!(
                            "Normalized clear_sky from {:.3} to {:.3}",
                            orig_clear, confidence.clear_sky
                        ),
                    });
                    actions.push(CorrectionAction {
                        triggered_by: violation.rule.clone(),
                        field: "precipitation".to_string(),
                        original_value: orig_precip,
                        corrected_value: confidence.precipitation,
                        reason: format!(
                            "Normalized precipitation from {:.3} to {:.3}",
                            orig_precip, confidence.precipitation
                        ),
                    });
                    actions.push(CorrectionAction {
                        triggered_by: violation.rule.clone(),
                        field: "storm".to_string(),
                        original_value: orig_storm,
                        corrected_value: confidence.storm,
                        reason: format!(
                            "Normalized storm from {:.3} to {:.3}",
                            orig_storm, confidence.storm
                        ),
                    });
                    actions.push(CorrectionAction {
                        triggered_by: violation.rule.clone(),
                        field: "calm".to_string(),
                        original_value: orig_calm,
                        corrected_value: confidence.calm,
                        reason: format!(
                            "Normalized calm from {:.3} to {:.3}",
                            orig_calm, confidence.calm
                        ),
                    });
                    actions.push(CorrectionAction {
                        triggered_by: violation.rule.clone(),
                        field: "windy".to_string(),
                        original_value: orig_windy,
                        corrected_value: confidence.windy,
                        reason: format!(
                            "Normalized windy from {:.3} to {:.3}",
                            orig_windy, confidence.windy
                        ),
                    });
                }
            }
        }

        actions
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
}
