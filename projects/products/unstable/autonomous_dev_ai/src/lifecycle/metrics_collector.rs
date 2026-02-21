// projects/products/unstable/autonomous_dev_ai/src/lifecycle/metrics_collector.rs
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
    time::{Duration, Instant},
};

use crate::lifecycle::{LifecycleMetrics, MetricsInner};

#[derive(Debug, Clone)]
pub struct MetricsCollector {
    inner: Arc<Mutex<MetricsInner>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MetricsInner {
                start_time: Instant::now(),
                iterations_total: 0,
                iterations_successful: 0,
                iterations_failed: 0,
                tool_executions: HashMap::new(),
                state_transitions: 0,
                risk_gate_allows: 0,
                risk_gate_denies: 0,
                risk_gate_high_approvals: 0,
                iteration_durations: Vec::new(),
            })),
        }
    }

    fn lock_inner(&self) -> MutexGuard<'_, MetricsInner> {
        match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    pub fn record_iteration_start(&self) {
        let mut inner = self.lock_inner();
        inner.iterations_total = inner.iterations_total.saturating_add(1);
    }

    pub fn record_iteration_success(&self, duration: Duration) {
        let mut inner = self.lock_inner();
        inner.iterations_successful = inner.iterations_successful.saturating_add(1);
        inner.iteration_durations.push(duration);
    }

    pub fn record_iteration_failure(&self, duration: Duration) {
        let mut inner = self.lock_inner();
        inner.iterations_failed = inner.iterations_failed.saturating_add(1);
        inner.iteration_durations.push(duration);
    }

    pub fn record_tool_execution(&self, tool_name: &str, success: bool, duration: Duration) {
        let mut inner = self.lock_inner();
        let metrics = inner
            .tool_executions
            .entry(tool_name.to_string())
            .or_default();

        metrics.executions = metrics.executions.saturating_add(1);
        if !success {
            metrics.failures = metrics.failures.saturating_add(1);
        }
        metrics.execution_times.push(duration);
    }

    pub fn record_state_transition(&self) {
        let mut inner = self.lock_inner();
        inner.state_transitions = inner.state_transitions.saturating_add(1);
    }

    pub fn record_risk_gate_allow(&self) {
        let mut inner = self.lock_inner();
        inner.risk_gate_allows = inner.risk_gate_allows.saturating_add(1);
    }

    pub fn record_risk_gate_deny(&self) {
        let mut inner = self.lock_inner();
        inner.risk_gate_denies = inner.risk_gate_denies.saturating_add(1);
    }

    pub fn record_risk_gate_high_approval(&self) {
        let mut inner = self.lock_inner();
        inner.risk_gate_high_approvals = inner.risk_gate_high_approvals.saturating_add(1);
    }

    pub fn snapshot(&self) -> LifecycleMetrics {
        let inner = self.lock_inner();

        let total_duration = inner.start_time.elapsed();
        let average_iteration_duration = if inner.iteration_durations.is_empty() {
            Duration::ZERO
        } else {
            let sum: Duration = inner.iteration_durations.iter().copied().sum();
            sum / (inner.iteration_durations.len() as u32)
        };

        let tool_execution_times = inner
            .tool_executions
            .iter()
            .map(|(name, metrics)| (name.clone(), metrics.execution_times.clone()))
            .collect();

        LifecycleMetrics {
            iterations_total: inner.iterations_total,
            iterations_successful: inner.iterations_successful,
            iterations_failed: inner.iterations_failed,
            tool_executions_total: inner.tool_executions.values().map(|m| m.executions).sum(),
            tool_executions_failed: inner.tool_executions.values().map(|m| m.failures).sum(),
            state_transitions_total: inner.state_transitions,
            risk_gate_allows: inner.risk_gate_allows,
            risk_gate_denies: inner.risk_gate_denies,
            risk_gate_high_approvals: inner.risk_gate_high_approvals,
            total_duration,
            average_iteration_duration,
            tool_execution_times,
        }
    }

    pub fn log_summary(&self) {
        let metrics = self.snapshot();

        tracing::info!("=== Lifecycle Metrics Summary ===");
        tracing::info!("Total duration: {:?}", metrics.total_duration);
        tracing::info!(
            "Iterations: {} total, {} successful, {} failed",
            metrics.iterations_total,
            metrics.iterations_successful,
            metrics.iterations_failed
        );
        tracing::info!(
            "Tool executions: {} total, {} failed",
            metrics.tool_executions_total,
            metrics.tool_executions_failed
        );
        tracing::info!("State transitions: {}", metrics.state_transitions_total);
        tracing::info!(
            "Risk gate: {} allows, {} denies, {} high-risk approvals",
            metrics.risk_gate_allows,
            metrics.risk_gate_denies,
            metrics.risk_gate_high_approvals
        );
        tracing::info!(
            "Average iteration duration: {:?}",
            metrics.average_iteration_duration
        );

        for (tool, times) in &metrics.tool_execution_times {
            if !times.is_empty() {
                let avg = times.iter().copied().sum::<Duration>() / (times.len() as u32);
                let min = times.iter().min().copied().unwrap_or(Duration::ZERO);
                let max = times.iter().max().copied().unwrap_or(Duration::ZERO);
                tracing::info!(
                    "Tool '{}': {} calls, avg={:?}, min={:?}, max={:?}",
                    tool,
                    times.len(),
                    avg,
                    min,
                    max
                );
            }
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}
