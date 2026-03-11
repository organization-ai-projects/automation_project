use crate::moe_core::{ExpertId, TaskId};

use super::feedback::{FeedbackEntry, FeedbackType};

#[derive(Debug, Clone)]
pub struct FeedbackStore {
    entries: Vec<FeedbackEntry>,
}

impl FeedbackStore {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, entry: FeedbackEntry) {
        self.entries.push(entry);
    }

    pub fn get_by_task(&self, task_id: &TaskId) -> Vec<&FeedbackEntry> {
        self.entries
            .iter()
            .filter(|e| e.task_id == *task_id)
            .collect()
    }

    pub fn get_by_expert(&self, expert_id: &ExpertId) -> Vec<&FeedbackEntry> {
        self.entries
            .iter()
            .filter(|e| e.expert_id == *expert_id)
            .collect()
    }

    pub fn get_by_type(&self, feedback_type: &FeedbackType) -> Vec<&FeedbackEntry> {
        self.entries
            .iter()
            .filter(|e| e.feedback_type == *feedback_type)
            .collect()
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn average_score_for_expert(&self, expert_id: &ExpertId) -> Option<f64> {
        let scores: Vec<f64> = self
            .entries
            .iter()
            .filter(|e| e.expert_id == *expert_id)
            .filter_map(|e| e.score)
            .collect();

        if scores.is_empty() {
            return None;
        }

        Some(scores.iter().sum::<f64>() / scores.len() as f64)
    }
}

impl Default for FeedbackStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_feedback(id: &str, task: &str, expert: &str, ft: FeedbackType) -> FeedbackEntry {
        FeedbackEntry {
            id: id.to_string(),
            task_id: TaskId::new(task),
            expert_id: ExpertId::new(expert),
            feedback_type: ft,
            score: Some(0.8),
            comment: "good".to_string(),
            created_at: 1,
        }
    }

    #[test]
    fn add_and_count() {
        let mut store = FeedbackStore::new();
        store.add(make_feedback("f1", "t1", "e1", FeedbackType::Positive));
        store.add(make_feedback("f2", "t1", "e1", FeedbackType::Negative));
        assert_eq!(store.count(), 2);
    }

    #[test]
    fn get_by_task() {
        let mut store = FeedbackStore::new();
        store.add(make_feedback("f1", "t1", "e1", FeedbackType::Positive));
        store.add(make_feedback("f2", "t2", "e1", FeedbackType::Positive));
        assert_eq!(store.get_by_task(&TaskId::new("t1")).len(), 1);
    }

    #[test]
    fn get_by_expert() {
        let mut store = FeedbackStore::new();
        store.add(make_feedback("f1", "t1", "e1", FeedbackType::Positive));
        store.add(make_feedback("f2", "t1", "e2", FeedbackType::Positive));
        assert_eq!(store.get_by_expert(&ExpertId::new("e1")).len(), 1);
    }

    #[test]
    fn get_by_type() {
        let mut store = FeedbackStore::new();
        store.add(make_feedback("f1", "t1", "e1", FeedbackType::Positive));
        store.add(make_feedback("f2", "t1", "e1", FeedbackType::Negative));
        assert_eq!(store.get_by_type(&FeedbackType::Positive).len(), 1);
    }

    #[test]
    fn average_score_for_expert() {
        let mut store = FeedbackStore::new();
        let mut fb1 = make_feedback("f1", "t1", "e1", FeedbackType::Positive);
        fb1.score = Some(0.8);
        let mut fb2 = make_feedback("f2", "t2", "e1", FeedbackType::Positive);
        fb2.score = Some(0.6);
        store.add(fb1);
        store.add(fb2);
        let avg = store
            .average_score_for_expert(&ExpertId::new("e1"))
            .unwrap();
        assert!((avg - 0.7).abs() < f64::EPSILON);
    }
}
