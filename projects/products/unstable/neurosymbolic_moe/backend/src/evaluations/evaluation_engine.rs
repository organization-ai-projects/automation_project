use std::collections::HashMap;

use crate::evaluations::{
    EvaluationGovernanceReport, ExpertMetrics, ExpertRegression, RoutingMetrics, RoutingRegression,
};
use crate::moe_core::ExpertId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn detect_expert_regressions(
        &self,
        baseline: &EvaluationEngine,
        min_drop: f64,
    ) -> Vec<ExpertRegression> {
        let mut regressions = Vec::new();

        for (expert_id, current) in &self.expert_metrics {
            if let Some(previous) = baseline.expert_metrics.get(expert_id) {
                let previous_success_rate = previous.success_rate();
                let current_success_rate = current.success_rate();
                let delta = current_success_rate - previous_success_rate;

                if delta <= -min_drop {
                    regressions.push(ExpertRegression {
                        expert_id: expert_id.clone(),
                        previous_success_rate,
                        current_success_rate,
                        delta,
                    });
                }
            }
        }

        regressions.sort_by(|a, b| {
            a.delta
                .partial_cmp(&b.delta)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        regressions
    }

    pub fn detect_routing_regression(
        &self,
        baseline: &EvaluationEngine,
        min_drop: f64,
    ) -> Option<RoutingRegression> {
        let previous_accuracy = baseline.routing_metrics.accuracy();
        let current_accuracy = self.routing_metrics.accuracy();
        let delta = current_accuracy - previous_accuracy;

        if delta <= -min_drop {
            Some(RoutingRegression {
                previous_accuracy,
                current_accuracy,
                delta,
            })
        } else {
            None
        }
    }

    pub fn governance_report(
        &self,
        min_expert_success_rate: f64,
        min_routing_accuracy: f64,
    ) -> EvaluationGovernanceReport {
        let underperforming_experts = self
            .expert_metrics
            .values()
            .filter(|metrics| metrics.total_executions > 0)
            .filter(|metrics| metrics.success_rate() < min_expert_success_rate)
            .map(|metrics| metrics.expert_id.clone())
            .collect::<Vec<_>>();

        let routing_accuracy_below_threshold = self.routing_metrics.total_routings > 0
            && self.routing_metrics.accuracy() < min_routing_accuracy;

        let ready_for_promotion =
            underperforming_experts.is_empty() && !routing_accuracy_below_threshold;

        EvaluationGovernanceReport {
            min_expert_success_rate,
            min_routing_accuracy,
            underperforming_experts,
            routing_accuracy_below_threshold,
            ready_for_promotion,
        }
    }

    pub fn checksum_fingerprint(&self) -> String {
        let mut experts = self
            .expert_metrics
            .values()
            .map(|m| {
                format!(
                    "{}:{}:{}:{}:{:.6}:{:.6}",
                    m.expert_id,
                    m.total_executions,
                    m.successful_executions,
                    m.failed_executions,
                    m.average_confidence,
                    m.average_latency_ms
                )
            })
            .collect::<Vec<_>>();
        experts.sort();

        format!(
            "experts=[{}];routing={}:{}:{}:{:.6}",
            experts.join("|"),
            self.routing_metrics.total_routings,
            self.routing_metrics.successful_routings,
            self.routing_metrics.fallback_count,
            self.routing_metrics.average_experts_per_task
        )
    }
}

impl Default for EvaluationEngine {
    fn default() -> Self {
        Self::new()
    }
}
