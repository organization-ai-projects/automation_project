use super::expert_verdict::ExpertVerdict;
use super::protection_action::ProtectionAction;

pub struct MoeAggregator;

impl MoeAggregator {
    pub fn aggregate(verdicts: &[ExpertVerdict]) -> (ProtectionAction, f64, String) {
        if verdicts.is_empty() {
            return (
                ProtectionAction::Log,
                0.0,
                "No expert analyzed this threat".to_string(),
            );
        }

        if verdicts.len() == 1 {
            let v = &verdicts[0];
            return (
                v.action.clone(),
                v.confidence,
                format!(
                    "Single expert verdict from {}: {}",
                    v.expert_id, v.reasoning
                ),
            );
        }

        // Confidence-weighted voting across experts
        let mut action_scores: Vec<(ProtectionAction, f64)> = Vec::new();

        for verdict in verdicts {
            if let Some(entry) = action_scores.iter_mut().find(|(a, _)| *a == verdict.action) {
                entry.1 += verdict.confidence;
            } else {
                action_scores.push((verdict.action.clone(), verdict.confidence));
            }
        }

        // Sort by score descending
        action_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let (best_action, best_score) = action_scores
            .first()
            .cloned()
            .unwrap_or((ProtectionAction::Log, 0.0));

        let total_confidence: f64 = verdicts.iter().map(|v| v.confidence).sum();
        let combined_confidence = if total_confidence > 0.0 {
            best_score / total_confidence
        } else {
            0.0
        };

        let summary = verdicts
            .iter()
            .map(|v| format!("{}: {:?} ({:.2})", v.expert_id, v.action, v.confidence))
            .collect::<Vec<_>>()
            .join("; ");

        (best_action, combined_confidence, summary)
    }
}
