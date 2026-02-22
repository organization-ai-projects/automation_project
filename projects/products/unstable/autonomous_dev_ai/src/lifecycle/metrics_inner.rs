// projects/products/unstable/autonomous_dev_ai/src/lifecycle/metrics_inner.rs
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::lifecycle::ToolMetrics;

#[derive(Debug)]
pub struct MetricsInner {
    pub start_time: Instant,
    pub iterations_total: usize,
    pub iterations_successful: usize,
    pub iterations_failed: usize,
    pub tool_executions: HashMap<String, ToolMetrics>,
    pub state_transitions: usize,
    pub risk_gate_allows: usize,
    pub risk_gate_denies: usize,
    pub risk_gate_high_approvals: usize,
    pub iteration_durations: Vec<Duration>,
}
