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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingMetrics {
    pub total_routings: u64,
    pub successful_routings: u64,
    pub fallback_count: u64,
    pub average_experts_per_task: f64,
}

impl RoutingMetrics {
    pub fn new() -> Self {
        Self {
            total_routings: 0,
            successful_routings: 0,
            fallback_count: 0,
            average_experts_per_task: 0.0,
        }
    }

    pub fn record_routing(&mut self, expert_count: usize, used_fallback: bool) {
        let prev_total = self.total_routings as f64;
        self.total_routings += 1;
        let new_total = self.total_routings as f64;

        if !used_fallback {
            self.successful_routings += 1;
        } else {
            self.fallback_count += 1;
        }

        self.average_experts_per_task =
            (self.average_experts_per_task * prev_total + expert_count as f64) / new_total;
    }

    pub fn accuracy(&self) -> f64 {
        if self.total_routings == 0 {
            return 0.0;
        }
        self.successful_routings as f64 / self.total_routings as f64
    }
}

impl Default for RoutingMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expert_metrics_record_and_success_rate() {
        let mut m = ExpertMetrics::new(ExpertId::new("e1"));
        assert_eq!(m.success_rate(), 0.0);

        m.record_execution(true, 0.9, 100.0);
        m.record_execution(true, 0.8, 200.0);
        m.record_execution(false, 0.5, 300.0);

        assert_eq!(m.total_executions, 3);
        assert_eq!(m.successful_executions, 2);
        assert_eq!(m.failed_executions, 1);
        assert!((m.success_rate() - 2.0 / 3.0).abs() < 1e-9);
        // Average confidence: (0.9 + 0.8 + 0.5) / 3 ≈ 0.7333
        assert!((m.average_confidence - (0.9 + 0.8 + 0.5) / 3.0).abs() < 1e-9);
    }

    #[test]
    fn routing_metrics_record() {
        let mut m = RoutingMetrics::new();
        assert_eq!(m.accuracy(), 0.0);

        m.record_routing(2, false);
        m.record_routing(1, true);

        assert_eq!(m.total_routings, 2);
        assert_eq!(m.successful_routings, 1);
        assert_eq!(m.fallback_count, 1);
        assert!((m.accuracy() - 0.5).abs() < f64::EPSILON);
        assert!((m.average_experts_per_task - 1.5).abs() < f64::EPSILON);
    }
}
