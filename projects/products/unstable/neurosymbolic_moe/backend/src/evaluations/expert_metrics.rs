use serde::{Deserialize, Serialize};

use crate::moe_core::ExpertId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertMetrics {
    pub expert_id: ExpertId,
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub average_confidence: f64,
    pub average_latency_ms: f64,
}

impl ExpertMetrics {
    pub fn new(expert_id: ExpertId) -> Self {
        Self {
            expert_id,
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            average_confidence: 0.0,
            average_latency_ms: 0.0,
        }
    }

    pub fn record_execution(&mut self, success: bool, confidence: f64, latency_ms: f64) {
        let prev_total = self.total_executions as f64;
        self.total_executions += 1;
        let new_total = self.total_executions as f64;

        if success {
            self.successful_executions += 1;
        } else {
            self.failed_executions += 1;
        }

        self.average_confidence = (self.average_confidence * prev_total + confidence) / new_total;
        self.average_latency_ms = (self.average_latency_ms * prev_total + latency_ms) / new_total;
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_executions == 0 {
            return 0.0;
        }
        self.successful_executions as f64 / self.total_executions as f64
    }
}
