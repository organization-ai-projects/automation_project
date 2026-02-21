// projects/products/unstable/autonomous_dev_ai/src/lifecycle/lifecycle_metrics.rs
use std::{collections::HashMap, time::Duration};

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
