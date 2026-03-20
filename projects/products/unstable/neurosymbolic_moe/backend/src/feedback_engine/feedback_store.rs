//! projects/products/unstable/neurosymbolic_moe/backend/src/feedback_engine/feedback_store.rs
use crate::{
    feedback_engine::{FeedbackEntry, FeedbackType},
    moe_core::{ExpertId, TaskId},
};

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
