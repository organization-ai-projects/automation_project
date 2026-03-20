use serde::{Deserialize, Serialize};

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
