// projects/products/unstable/autonomous_dev_ai/src/lifecycle/resource_budget.rs
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Resource limits enforced during autonomous execution.
/// When any budget is exceeded the agent must transition to a fail-safe state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceBudget {
    /// Maximum wall-clock runtime for the entire run.
    pub max_runtime: Duration,
    /// Maximum cumulative process CPU time for the entire run.
    pub max_cpu_time: Duration,
    /// Maximum resident set size in bytes for the running process.
    pub max_rss_bytes: usize,
    /// Maximum number of lifecycle iterations.
    pub max_iterations: usize,
    /// Maximum number of tool executions across the run.
    pub max_tool_executions: usize,
}

impl ResourceBudget {
    pub fn new(
        max_runtime: Duration,
        max_cpu_time: Duration,
        max_rss_bytes: usize,
        max_iterations: usize,
        max_tool_executions: usize,
    ) -> Self {
        Self {
            max_runtime,
            max_cpu_time,
            max_rss_bytes,
            max_iterations,
            max_tool_executions,
        }
    }

    /// Check whether any budget limit has been breached.
    pub fn is_exceeded(
        &self,
        elapsed: Duration,
        iterations: usize,
        tool_executions: usize,
    ) -> Option<&'static str> {
        if elapsed >= self.max_runtime {
            return Some("runtime budget exceeded");
        }
        if iterations >= self.max_iterations {
            return Some("iteration budget exceeded");
        }
        if tool_executions >= self.max_tool_executions {
            return Some("tool execution budget exceeded");
        }
        None
    }
}

impl Default for ResourceBudget {
    fn default() -> Self {
        Self::new(
            Duration::from_secs(3600),
            Duration::from_secs(1800),
            2 * 1024 * 1024 * 1024,
            100,
            500,
        )
    }
}
