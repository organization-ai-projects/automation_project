use std::collections::HashMap;

use crate::moe_core::{ExpertId, TaskId};
use serde::{Deserialize, Serialize};

use super::{Correction, DatasetEntry, Outcome};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetQualityReport {
    pub total_entries: usize,
    pub scored_entries: usize,
    pub average_score: Option<f64>,
    pub low_score_entries: usize,
    pub corrected_entries: usize,
    pub correction_ratio: f64,
    pub success_ratio: f64,
}

#[derive(Debug, Clone)]
pub struct DatasetStore {
    entries: Vec<DatasetEntry>,
    corrections: HashMap<String, Vec<Correction>>,
}

impl DatasetStore {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            corrections: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, entry: DatasetEntry) {
        self.entries.push(entry);
    }

    pub fn add_correction(&mut self, correction: Correction) {
        self.corrections
            .entry(correction.entry_id.clone())
            .or_default()
            .push(correction);
    }

    pub fn get_by_task(&self, task_id: &TaskId) -> Vec<&DatasetEntry> {
        self.entries
            .iter()
            .filter(|e| e.task_id == *task_id)
            .collect()
    }

    pub fn get_by_expert(&self, expert_id: &ExpertId) -> Vec<&DatasetEntry> {
        self.entries
            .iter()
            .filter(|e| e.expert_id == *expert_id)
            .collect()
    }

    pub fn get_by_outcome(&self, outcome: &Outcome) -> Vec<&DatasetEntry> {
        self.entries
            .iter()
            .filter(|e| e.outcome == *outcome)
            .collect()
    }

    pub fn get_corrections(&self, entry_id: &str) -> Option<&Vec<Correction>> {
        self.corrections.get(entry_id)
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn successful_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.outcome == Outcome::Success)
            .count()
    }

    pub fn failed_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.outcome == Outcome::Failure)
            .count()
    }

    pub fn average_score(&self) -> Option<f64> {
        let mut total = 0.0;
        let mut count = 0usize;

        for entry in &self.entries {
            if let Some(score) = entry.score {
                total += score;
                count += 1;
            }
        }

        if count == 0 {
            None
        } else {
            Some(total / count as f64)
        }
    }

    pub fn quality_report(&self, low_score_threshold: f64) -> DatasetQualityReport {
        let total_entries = self.entries.len();
        let scored_entries = self
            .entries
            .iter()
            .filter(|entry| entry.score.is_some())
            .count();
        let average_score = self.average_score();
        let low_score_entries = self
            .entries
            .iter()
            .filter(|entry| entry.score.is_some_and(|score| score < low_score_threshold))
            .count();

        let corrected_entries = self
            .entries
            .iter()
            .filter(|entry| {
                self.corrections
                    .get(&entry.id)
                    .is_some_and(|corrections| !corrections.is_empty())
            })
            .count();

        let correction_ratio = if total_entries == 0 {
            0.0
        } else {
            corrected_entries as f64 / total_entries as f64
        };

        let success_ratio = if total_entries == 0 {
            0.0
        } else {
            self.successful_count() as f64 / total_entries as f64
        };

        DatasetQualityReport {
            total_entries,
            scored_entries,
            average_score,
            low_score_entries,
            corrected_entries,
            correction_ratio,
            success_ratio,
        }
    }
}

impl Default for DatasetStore {
    fn default() -> Self {
        Self::new()
    }
}
