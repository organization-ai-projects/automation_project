use std::collections::HashMap;

use crate::moe_core::{TaskId, TracePhase, TraceRecord};
use crate::trace_logger::TraceLogger;

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
    assert!(logger.get_by_task(&TaskId::new("t1")).is_empty());
    assert_eq!(logger.get_by_task(&TaskId::new("t3")).len(), 1);
}

#[test]
fn get_by_task_returns_only_matching_traces() {
    let mut logger = TraceLogger::new(100);
    logger.log(make_record("t1", TracePhase::Routing));
    logger.log(make_record("t1", TracePhase::ExpertExecution));
    logger.log(make_record("t2", TracePhase::Routing));
    assert_eq!(logger.get_by_task(&TaskId::new("t1")).len(), 2);
    assert_eq!(logger.get_by_task(&TaskId::new("t2")).len(), 1);
}
