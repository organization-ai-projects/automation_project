use std::collections::HashMap;

use crate::moe_core::{ExpertId, TaskId};

use super::{Correction, DatasetEntry, Outcome};

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
}

impl Default for DatasetStore {
    fn default() -> Self {
        Self::new()
    }
}
