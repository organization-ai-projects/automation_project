//! projects/products/unstable/autonomous_dev_ai/backend/src/resources/checks.rs
use crate::{
    lifecycle::{
        IterationNumber, LifecycleError, LifecycleResult, MetricsCollector, ProcessUsage,
        ResourceBudget, ResourceType,
    },
    memory_graph::MemoryGraph,
    ops::RunReplay,
    timeout::Timeout,
};
use std::{env, time::Instant};

pub struct ResourceChecker {
    pub global_timeout: Timeout,
    pub metrics: MetricsCollector,
    pub resource_budget: ResourceBudget,
    pub memory: MemoryGraph,
    pub iteration: usize,
    pub current_iteration_number: IterationNumber,
    pub run_replay: RunReplay,
}

impl ResourceChecker {
    pub(crate) fn check_resource_budgets(&mut self, start_time: Instant) -> LifecycleResult<()> {
        if start_time.elapsed() > self.global_timeout.duration {
            tracing::error!("Global timeout exceeded: {:?}", start_time.elapsed());
            self.metrics.record_iteration_failure(start_time.elapsed());

            return Err(LifecycleError::Timeout {
                iteration: self.iteration,
                elapsed: start_time.elapsed(),
                limit: self.global_timeout,
            });
        }

        let metrics_snapshot = self.metrics.snapshot();
        if let Some(process_usage) = ProcessUsage::sample() {
            if process_usage.cpu_time >= self.resource_budget.max_cpu_time {
                self.record_resource_budget_failure(
                    format!(
                        "cpu budget exceeded: cpu_secs={} budget_secs={}",
                        process_usage.cpu_time.as_secs(),
                        self.resource_budget.max_cpu_time.as_secs()
                    ),
                    Some("reduce workload or increase AUTONOMOUS_MAX_CPU_SECONDS".to_string()),
                );
                self.transition_to_failed_state()?;
                return Err(LifecycleError::ResourceExhausted {
                    resource: ResourceType::CpuTime,
                    limit: self.resource_budget.max_cpu_time.as_secs() as usize,
                    current: process_usage.cpu_time.as_secs() as usize,
                });
            }

            if process_usage.rss_bytes >= self.resource_budget.max_rss_bytes {
                self.record_resource_budget_failure(
                    format!(
                        "rss budget exceeded: rss_bytes={} budget_bytes={}",
                        process_usage.rss_bytes, self.resource_budget.max_rss_bytes
                    ),
                    Some("reduce memory pressure or increase AUTONOMOUS_MAX_RSS_MB".to_string()),
                );
                self.transition_to_failed_state()?;
                return Err(LifecycleError::ResourceExhausted {
                    resource: ResourceType::Memory,
                    limit: self.resource_budget.max_rss_bytes,
                    current: process_usage.rss_bytes,
                });
            }
        } else {
            self.run_replay.record(
                "runtime.process_usage.unavailable",
                "CPU/RSS sampling unavailable; skipping cpu/rss budget checks",
            );
        }

        let memory_entries = self.memory.explored_files.len()
            + self.memory.plans.len()
            + self.memory.decisions.len()
            + self.memory.failures.len()
            + self.memory.objective_evaluations.len();
        let memory_budget = env::var("AUTONOMOUS_MAX_MEMORY_ENTRIES")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(10_000);
        if memory_entries >= memory_budget {
            self.record_resource_budget_failure(
                format!(
                    "memory budget exceeded: entries={} budget={}",
                    memory_entries, memory_budget
                ),
                Some(
                    "reduce retained memory or increase AUTONOMOUS_MAX_MEMORY_ENTRIES".to_string(),
                ),
            );
            self.transition_to_failed_state()?;
            return Err(LifecycleError::ResourceExhausted {
                resource: ResourceType::Memory,
                limit: memory_budget,
                current: memory_entries,
            });
        }

        if let Some(limit_reason) = self.resource_budget.is_exceeded(
            start_time.elapsed(),
            self.current_iteration_number.get(),
            metrics_snapshot.tool_executions_total,
        ) {
            let resource = match limit_reason {
                "runtime budget exceeded" => ResourceType::Time,
                "tool execution budget exceeded" => ResourceType::ToolExecutions,
                _ => ResourceType::Iterations,
            };
            self.record_resource_budget_failure(
                limit_reason.to_string(),
                Some("reduce run scope or increase configured budget".to_string()),
            );
            self.transition_to_failed_state()?;
            return Err(LifecycleError::ResourceExhausted {
                resource,
                limit: match resource {
                    ResourceType::Time => self.resource_budget.max_runtime.as_secs() as usize,
                    ResourceType::CpuTime => self.resource_budget.max_cpu_time.as_secs() as usize,
                    ResourceType::Memory => self.resource_budget.max_rss_bytes,
                    ResourceType::ToolExecutions => self.resource_budget.max_tool_executions,
                    _ => self.resource_budget.max_iterations,
                },
                current: match resource {
                    ResourceType::Time => start_time.elapsed().as_secs() as usize,
                    ResourceType::CpuTime => ProcessUsage::sample()
                        .map(|usage| usage.cpu_time.as_secs() as usize)
                        .unwrap_or_default(),
                    ResourceType::Memory => ProcessUsage::sample()
                        .map(|usage| usage.rss_bytes)
                        .unwrap_or_default(),
                    ResourceType::ToolExecutions => metrics_snapshot.tool_executions_total,
                    _ => self.current_iteration_number.get(),
                },
            });
        }

        self.check_iteration_budget()?;

        Ok(())
    }

    fn record_resource_budget_failure(&self, error: String, recovery: Option<String>) {
        tracing::error!("Resource budget failure: {}", error);
        if let Some(recovery) = recovery {
            tracing::info!("Recovery suggestion: {}", recovery);
        }
    }

    fn transition_to_failed_state(&self) -> LifecycleResult<()> {
        tracing::warn!("Transitioning to failed state");
        Ok(())
    }

    fn check_iteration_budget(&self) -> LifecycleResult<()> {
        tracing::info!("Checking iteration budget");
        Ok(())
    }
}
