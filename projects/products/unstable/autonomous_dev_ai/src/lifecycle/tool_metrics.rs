//projects/products/unstable/autonomous_dev_ai/src/lifecycle/tool_metrics.rs
use std::time::Duration;

#[derive(Debug, Default)]
pub struct ToolMetrics {
    pub executions: usize,
    pub failures: usize,
    pub execution_times: Vec<Duration>,
}
