//! Metrics collection for lifecycle observability.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Default)]
pub struct LifecycleMetrics {
    pub iterations_total: usize,
    pub iterations_successful: usize,
    pub iterations_failed: usize,
    pub tool_executions_total: usize,
    pub tool_executions_failed: usize,
    pub state_transitions_total: usize,
    pub total_duration: Duration,
    pub average_iteration_duration: Duration,
    pub tool_execution_times: HashMap<String, Vec<Duration>>,
}

#[derive(Debug, Clone)]
pub struct MetricsCollector {
    inner: Arc<Mutex<MetricsInner>>,
}

#[derive(Debug)]
struct MetricsInner {
    start_time: Instant,
    iterations_total: usize,
    iterations_successful: usize,
    iterations_failed: usize,
    tool_executions: HashMap<String, ToolMetrics>,
    state_transitions: usize,
    iteration_durations: Vec<Duration>,
}

#[derive(Debug, Default)]
struct ToolMetrics {
    executions: usize,
    failures: usize,
    execution_times: Vec<Duration>,
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
