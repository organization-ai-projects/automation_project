use std::collections::HashMap;

use crate::moe_core::ExpertId;

use super::metrics::{ExpertMetrics, RoutingMetrics};

#[derive(Debug, Clone)]
pub struct EvaluationEngine {
    expert_metrics: HashMap<ExpertId, ExpertMetrics>,
    routing_metrics: RoutingMetrics,
}

impl EvaluationEngine {
    pub fn new() -> Self {
        Self {
            expert_metrics: HashMap::new(),
            routing_metrics: RoutingMetrics::new(),
        }
    }

    pub fn record_expert_execution(
        &mut self,
        expert_id: ExpertId,
        success: bool,
        confidence: f64,
        latency_ms: f64,
    ) {
        let metrics = self
            .expert_metrics
            .entry(expert_id.clone())
            .or_insert_with(|| ExpertMetrics::new(expert_id));
        metrics.record_execution(success, confidence, latency_ms);
    }

    pub fn record_routing(&mut self, expert_count: usize, used_fallback: bool) {
        self.routing_metrics
            .record_routing(expert_count, used_fallback);
    }

    pub fn get_expert_metrics(&self, expert_id: &ExpertId) -> Option<&ExpertMetrics> {
        self.expert_metrics.get(expert_id)
    }

    pub fn get_routing_metrics(&self) -> &RoutingMetrics {
        &self.routing_metrics
    }

    pub fn best_performing_expert(&self) -> Option<&ExpertMetrics> {
        self.expert_metrics.values().max_by(|a, b| {
            a.success_rate()
                .partial_cmp(&b.success_rate())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn worst_performing_expert(&self) -> Option<&ExpertMetrics> {
        self.expert_metrics.values().min_by(|a, b| {
            a.success_rate()
                .partial_cmp(&b.success_rate())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

impl Default for EvaluationEngine {
    fn default() -> Self {
        Self::new()
    }
}
