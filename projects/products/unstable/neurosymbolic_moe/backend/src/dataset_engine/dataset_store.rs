use std::collections::HashMap;

use crate::moe_core::{ExpertId, TaskId};

use super::dataset_entry::{Correction, DatasetEntry, Outcome};

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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(id: &str, task: &str, expert: &str, outcome: Outcome) -> DatasetEntry {
        DatasetEntry {
            id: id.to_string(),
            task_id: TaskId::new(task),
            expert_id: ExpertId::new(expert),
            input: "input".to_string(),
            output: "output".to_string(),
            outcome,
            score: Some(0.9),
            tags: vec!["tag1".to_string()],
            created_at: 1,
            metadata: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn add_entry_and_count() {
        let mut store = DatasetStore::new();
        store.add_entry(make_entry("d1", "t1", "e1", Outcome::Success));
        store.add_entry(make_entry("d2", "t1", "e2", Outcome::Failure));
        assert_eq!(store.count(), 2);
    }

    #[test]
    fn get_by_task() {
        let mut store = DatasetStore::new();
        store.add_entry(make_entry("d1", "t1", "e1", Outcome::Success));
        store.add_entry(make_entry("d2", "t2", "e1", Outcome::Success));
        let results = store.get_by_task(&TaskId::new("t1"));
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn get_by_expert() {
        let mut store = DatasetStore::new();
        store.add_entry(make_entry("d1", "t1", "e1", Outcome::Success));
        store.add_entry(make_entry("d2", "t2", "e2", Outcome::Success));
        let results = store.get_by_expert(&ExpertId::new("e1"));
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn get_by_outcome() {
        let mut store = DatasetStore::new();
        store.add_entry(make_entry("d1", "t1", "e1", Outcome::Success));
        store.add_entry(make_entry("d2", "t2", "e2", Outcome::Failure));
        assert_eq!(store.successful_count(), 1);
        assert_eq!(store.failed_count(), 1);
        let successes = store.get_by_outcome(&Outcome::Success);
        assert_eq!(successes.len(), 1);
    }
}
