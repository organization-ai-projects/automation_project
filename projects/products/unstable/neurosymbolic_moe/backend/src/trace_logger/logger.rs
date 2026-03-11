//! projects/products/unstable/neurosymbolic_moe/backend/src/trace_logger/logger.rs
use std::collections::HashMap;

use crate::moe_core::{ExpertId, TaskId, TracePhase, TraceRecord};

#[derive(Debug, Clone)]
pub struct TraceLogger {
    traces: Vec<TraceRecord>,
    max_traces: usize,
    counter: u64,
}

impl TraceLogger {
    pub fn new(max_traces: usize) -> Self {
        Self {
            traces: Vec::new(),
            max_traces,
            counter: 0,
        }
    }

    pub fn log(&mut self, record: TraceRecord) {
        if self.traces.len() >= self.max_traces {
            self.traces.remove(0);
        }
        self.traces.push(record);
    }

    pub fn log_phase(
        &mut self,
        task_id: TaskId,
        phase: TracePhase,
        detail: String,
        expert_id: Option<ExpertId>,
    ) {
        self.counter += 1;

        let record = TraceRecord {
            trace_id: format!("trace-{}-{}", task_id.as_str(), self.counter),
            task_id,
            // Deterministic monotonic timestamp surrogate.
            timestamp: self.counter,
            expert_id,
            phase,
            detail,
            metadata: HashMap::new(),
        };
        self.log(record);
    }

    pub fn get_by_task(&self, task_id: &TaskId) -> Vec<&TraceRecord> {
        self.traces
            .iter()
            .filter(|t| t.task_id == *task_id)
            .collect()
    }

    pub fn get_by_phase(&self, phase: &TracePhase) -> Vec<&TraceRecord> {
        self.traces
            .iter()
            .filter(|t| std::mem::discriminant(&t.phase) == std::mem::discriminant(phase))
            .collect()
    }

    pub fn get_by_expert(&self, expert_id: &ExpertId) -> Vec<&TraceRecord> {
        self.traces
            .iter()
            .filter(|t| t.expert_id.as_ref() == Some(expert_id))
            .collect()
    }

    pub fn recent(&self, count: usize) -> Vec<&TraceRecord> {
        self.traces.iter().rev().take(count).collect()
    }

    pub fn count(&self) -> usize {
        self.traces.len()
    }

    pub fn clear(&mut self) {
        self.traces.clear();
    }
}
