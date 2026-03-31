use crate::decision::DecisionSummary;
use crate::decision::decision_confidence::DecisionConfidence;
use crate::journal::{DecisionEntry, ThesisSnapshot};

pub struct JournalEngine;

impl JournalEngine {
    pub fn record_decision(
        ticker: &str,
        timestamp: &str,
        summary: &DecisionSummary,
    ) -> DecisionEntry {
        let rationale = summary
            .primary_reasons
            .iter()
            .map(|r| r.description.clone())
            .collect::<Vec<_>>()
            .join("; ");

        DecisionEntry::new(
            timestamp,
            ticker,
            summary.recommended_action.clone(),
            DecisionConfidence::from_score(summary.confidence.score),
            rationale,
            summary.invalidation_conditions.clone(),
        )
    }

    pub fn create_thesis_snapshot(
        ticker: &str,
        date: &str,
        thesis: &str,
        assumptions: Vec<String>,
        triggers: Vec<String>,
    ) -> ThesisSnapshot {
        let mut snap = ThesisSnapshot::new(ticker, date, thesis);
        snap.key_assumptions = assumptions;
        snap.invalidation_triggers = triggers;
        snap
    }
}
