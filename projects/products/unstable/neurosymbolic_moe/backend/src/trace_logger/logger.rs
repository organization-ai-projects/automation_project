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
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let record = TraceRecord {
            trace_id: format!("trace-{}-{}", task_id.as_str(), self.counter),
            task_id,
            timestamp,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_record(task: &str, phase: TracePhase) -> TraceRecord {
        TraceRecord {
            trace_id: format!("tr-{task}"),
            task_id: TaskId::new(task),
            timestamp: 1,
            expert_id: None,
            phase,
            detail: "detail".to_string(),
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn log_and_count() {
        let mut logger = TraceLogger::new(100);
        logger.log(make_record("t1", TracePhase::Routing));
        logger.log(make_record("t2", TracePhase::ExpertExecution));
        assert_eq!(logger.count(), 2);
    }

    #[test]
    fn max_traces_eviction() {
        let mut logger = TraceLogger::new(2);
        logger.log(make_record("t1", TracePhase::Routing));
        logger.log(make_record("t2", TracePhase::Routing));
        logger.log(make_record("t3", TracePhase::Routing));
        assert_eq!(logger.count(), 2);
        // First record (t1) should have been evicted
        assert!(logger.get_by_task(&TaskId::new("t1")).is_empty());
        assert_eq!(logger.get_by_task(&TaskId::new("t3")).len(), 1);
    }

    #[test]
    fn get_by_task() {
        let mut logger = TraceLogger::new(100);
        logger.log(make_record("t1", TracePhase::Routing));
        logger.log(make_record("t1", TracePhase::ExpertExecution));
        logger.log(make_record("t2", TracePhase::Routing));
        assert_eq!(logger.get_by_task(&TaskId::new("t1")).len(), 2);
        assert_eq!(logger.get_by_task(&TaskId::new("t2")).len(), 1);
    }
}
